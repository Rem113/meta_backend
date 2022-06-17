use std::{collections::HashMap, sync::Arc, time::Duration};

use bollard::{
    container::{
        Config, CreateContainerOptions, LogOutput, LogsOptions, RemoveContainerOptions,
        StartContainerOptions,
    },
    models::HostConfig,
    Docker,
};
use futures::{future::try_join_all, SinkExt, StreamExt};
use mongodb::bson::oid::ObjectId;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{trace, warn};

use warp::ws::Message;

use crate::{
    data::{Environment, Image, Repository, Scenario, Simulator, Step},
    domain::running_simulator::RunningSimulator,
};

use super::error::Error;

pub struct DockerExecutor {}

impl DockerExecutor {
    pub async fn run_scenario_in_environment(
        docker: Arc<Docker>,
        environment: &Environment,
        scenario: &Scenario,
        repository: Repository,
        mut web_socket: warp::ws::WebSocket,
    ) -> Result<(), Error> {
        let steps = scenario.steps();

        let unique_simulators = steps.iter().fold(Vec::new(), |mut accumulator, step| {
            if !accumulator.contains(&step.simulator_id) {
                accumulator.push(step.simulator_id);
            }
            accumulator
        });

        let simulator_id_to_running_simulator =
            instantiate_simulators(unique_simulators, repository, docker.clone(), environment)
                .await?;

        let running_simulators = simulator_id_to_running_simulator
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        attach_log_listener_to_simulators(docker.clone(), running_simulators.clone(), tx.clone())?;

        wait_for_simulators_to_be_ready(running_simulators.clone()).await?;

        tokio::spawn(async move {
            while let Some(log) = rx.recv().await {
                if let Err(err) = web_socket.send(Message::text(format!("{:?}", log))).await {
                    warn!("{:?}", err);
                }
            }
        });

        let step_data = steps
            .iter()
            .map(|step| {
                (
                    step,
                    simulator_id_to_running_simulator
                        .get(&step.simulator_id)
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        if let Err(error) = run_scenario(&step_data).await {
            tx.send(LogOutput::StdErr {
                message: error.to_string().into(),
            })
            .ok();
        }

        for running_simulator in running_simulators {
            docker
                .remove_container(
                    running_simulator.name(),
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await?;
        }

        Ok(())
    }
}

async fn instantiate_simulators(
    simulators: Vec<ObjectId>,
    repository: Repository,
    docker: Arc<Docker>,
    environment: &Environment,
) -> Result<HashMap<ObjectId, RunningSimulator>, Error> {
    let mut simulator_id_to_running_simulator = HashMap::new();

    for simulator_id in simulators {
        let simulator = repository.find_by_id::<Simulator>(&simulator_id).await?;

        let simulator = match simulator {
            Some(simulator) => simulator,
            None => return Err(Error::SimulatorNotFound(simulator_id.to_string())),
        };

        let image = repository.find_by_id(simulator.image_id()).await?;

        let image = match image {
            Some(image) => image,
            None => return Err(Error::ImageNotFound(simulator.image_id().to_string())),
        };

        let running_simulator =
            instantiate_simulator(docker.clone(), environment, &simulator, image).await?;

        simulator_id_to_running_simulator.insert(simulator_id, running_simulator);
    }

    Ok(simulator_id_to_running_simulator)
}

async fn instantiate_simulator(
    docker: Arc<Docker>,
    environment: &Environment,
    simulator: &Simulator,
    image: Image,
) -> Result<RunningSimulator, Error> {
    let container_name = create_container(&docker.clone(), environment, simulator, image).await?;

    docker
        .start_container(&container_name, None::<StartContainerOptions<String>>)
        .await?;

    let port = get_exposed_port_for_container(docker.clone(), &container_name).await?;

    trace!("Container exposes port {}", port);

    Ok(RunningSimulator::new(container_name.clone(), port))
}

async fn create_container(
    docker: &Arc<Docker>,
    environment: &Environment,
    simulator: &Simulator,
    image: Image,
) -> Result<String, Error> {
    let name = format!("{}-{}", environment.name(), simulator.name());

    let container_create_response = docker
        .create_container(
            Some(CreateContainerOptions { name: &name }),
            Config {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                image: Some(image.tag().as_meta()),
                host_config: Some(HostConfig {
                    publish_all_ports: Some(true),
                    ..Default::default()
                }),
                env: Some(
                    simulator
                        .configuration()
                        .iter()
                        .map(|(key, value)| format!("{}={}", key, value))
                        .collect::<Vec<String>>(),
                ),
                ..Default::default()
            },
        )
        .await?;

    trace!("{:?}", container_create_response);

    Ok(name)
}

async fn get_exposed_port_for_container(
    docker: Arc<Docker>,
    container_name: &str,
) -> Result<u16, Error> {
    let container_inspect_response = docker.inspect_container(container_name, None).await?;

    let port = container_inspect_response
        .network_settings
        .and_then(|network_settings| network_settings.ports)
        .and_then(|ports| ports.get("3000/tcp").cloned())
        .and_then(|port| port.as_ref().cloned())
        .and_then(|ports| {
            ports
                .into_iter()
                .filter_map(|port| port.host_port)
                .collect::<Vec<_>>()
                .first()
                .cloned()
        })
        .and_then(|port| port.parse::<u16>().ok());

    match port {
        Some(port) => Ok(port),
        None => Err(Error::SimulatorNotFound(format!(
            "Could not find exposed port for simulator {}",
            container_name
        ))),
    }
}

fn attach_log_listener_to_simulators(
    docker: Arc<Docker>,
    running_simulators: Vec<RunningSimulator>,
    tx: Arc<UnboundedSender<LogOutput>>,
) -> Result<(), Error> {
    for running_simulator in running_simulators {
        let docker = docker.clone();
        let tx = tx.clone();

        tokio::spawn(async move {
            docker
                .logs(
                    running_simulator.name(),
                    Some(LogsOptions::<String> {
                        follow: true,
                        stdout: true,
                        stderr: true,
                        timestamps: true,
                        ..Default::default()
                    }),
                )
                .filter_map(|result| async { result.ok() })
                .for_each(|log| async {
                    tx.send(log).ok();
                })
                .await;
        });
    }

    Ok(())
}

async fn wait_for_simulators_to_be_ready(
    running_simulators: Vec<RunningSimulator>,
) -> Result<(), Error> {
    let ready_futures = running_simulators
        .into_iter()
        .map(|running_simulator| {
            tokio::spawn(async move {
                for _ in 0..30 {
                    if running_simulator.is_ready().await {
                        return Ok(());
                    }

                    tokio::time::sleep(Duration::from_secs(1)).await;
                }

                Err(Error::SimulatorNotReady(format!(
                    "Simulator {} was not ready within 30 seconds",
                    running_simulator.name()
                )))
            })
        })
        .collect::<Vec<_>>();

    let result = try_join_all(ready_futures).await.map_err(|error| {
        Error::SimulatorNotReady(format!(
            "Error while waiting for simulators to be ready: {}",
            error
        ))
    })?;

    result.into_iter().collect()
}

async fn run_scenario(step_data: &[(&Step, &RunningSimulator)]) -> Result<(), Error> {
    for (step, running_simulator) in step_data.iter() {
        trace!(
            "Command: {:?}, Arguments: {:?}",
            step.command,
            step.arguments
        );

        let command_result = running_simulator
            .execute_command(&step.command.path, &step.arguments)
            .await;

        if let Err(err) = command_result {
            return Err(Error::SimulatorCommand(format!(
                "Step failed with error: {:?}",
                err
            )));
        }

        trace!("Ran command {:?}", step.command);
    }

    Ok(())
}
