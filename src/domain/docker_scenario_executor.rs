use std::{collections::HashMap, sync::Arc, time::Duration};

use bollard::{container::LogOutput, Docker};
use futures::{future::try_join_all, SinkExt};
use mongodb::bson::oid::ObjectId;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{trace, warn};

use warp::ws::Message;

use crate::{
    data::{Environment, Repository, Scenario, Simulator, Step},
    domain::{docker_simulator::DockerSimulator, running_docker_simulator::RunningDockerSimulator},
};

use super::error::Error;

pub struct DockerScenarioExecutor {}

impl DockerScenarioExecutor {
    pub async fn run_scenario_in_environment(
        docker: Arc<Docker>,
        environment: &Environment,
        scenario: &Scenario,
        repository: Repository,
        mut web_socket: warp::ws::WebSocket,
    ) -> Result<(), Error> {
        let steps = scenario.steps();

        let mut unique_simulators = steps
            .iter()
            .map(|step| step.simulator_id)
            .collect::<Vec<_>>();
        unique_simulators.dedup();

        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        let simulator_id_to_running_simulator = instantiate_simulators(
            unique_simulators,
            repository,
            docker.clone(),
            environment,
            tx.clone(),
        )
        .await?;

        let running_simulators = simulator_id_to_running_simulator
            .values()
            .cloned()
            .collect::<Vec<_>>();

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

        for running_docker_simulator in running_simulators {
            running_docker_simulator.remove().await?;
        }

        Ok(())
    }
}

async fn instantiate_simulators(
    simulators: Vec<ObjectId>,
    repository: Repository,
    docker: Arc<Docker>,
    environment: &Environment,
    tx: Arc<UnboundedSender<LogOutput>>,
) -> Result<HashMap<ObjectId, RunningDockerSimulator>, Error> {
    let mut simulator_id_to_running_docker_simulator = HashMap::new();

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

        let docker_container =
            DockerSimulator::create(docker.clone(), environment, &simulator, &image).await?;

        let docker_simulator = docker_container.start(Some(tx.clone())).await?;

        simulator_id_to_running_docker_simulator.insert(simulator_id, docker_simulator);
    }

    Ok(simulator_id_to_running_docker_simulator)
}

async fn wait_for_simulators_to_be_ready(
    running_docker_simulators: Vec<RunningDockerSimulator>,
) -> Result<(), Error> {
    let ready_futures = running_docker_simulators
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

async fn run_scenario(step_data: &[(&Step, &RunningDockerSimulator)]) -> Result<(), Error> {
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
