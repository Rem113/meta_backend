use std::{collections::HashMap, sync::Arc, time::Duration};

use bollard::Docker;
use futures::{future::try_join_all, SinkExt};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{trace, warn};
use warp::ws::Message;

use crate::domain::scenario_playing_event::ScenarioPlayingEvent;
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

        let mut unique_images = steps.iter().map(|step| step.image_id).collect::<Vec<_>>();
        unique_images.dedup();

        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        let image_id_to_running_simulator = instantiate_simulators(
            unique_images,
            repository,
            docker.clone(),
            environment,
            tx.clone(),
        )
        .await?;

        let running_simulators = image_id_to_running_simulator
            .values()
            .cloned()
            .collect::<Vec<_>>();

        wait_for_simulators_to_be_ready(running_simulators.clone()).await?;

        tokio::spawn(async move {
            while let Some(log) = rx.recv().await {
                if let Err(err) = web_socket
                    .send(Message::text(
                        serde_json::to_string(&log).unwrap_or_default(),
                    ))
                    .await
                {
                    warn!("{:?}", err);
                }
            }
        });

        let step_data = steps
            .iter()
            .map(|step| {
                (
                    step,
                    image_id_to_running_simulator.get(&step.image_id).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        if let Err(error) = run_scenario(&step_data, tx.clone()).await {
            match error {
                Error::SimulatorCommandFailed { message, status } => tx
                    .send(ScenarioPlayingEvent::StepFailed {
                        message,
                        status: status.as_u16(),
                    })
                    .ok(),
                _ => Some(()),
            };
        }

        for running_docker_simulator in running_simulators {
            if let Err(error) = running_docker_simulator.remove().await {
                warn!("Failed to remove simulator: {:?}", error);
            }
        }

        Ok(())
    }
}

async fn instantiate_simulators(
    images: Vec<ObjectId>,
    repository: Repository,
    docker: Arc<Docker>,
    environment: &Environment,
    tx: Arc<UnboundedSender<ScenarioPlayingEvent>>,
) -> Result<HashMap<ObjectId, RunningDockerSimulator>, Error> {
    let mut image_id_to_running_docker_simulator = HashMap::new();

    for image_id in images {
        let simulator = repository
            .find::<Simulator>(doc! {
                "imageId": image_id,
                "environmentId": environment.id().unwrap()
            })
            .await?;

        let simulator = match simulator.first() {
            Some(simulator) => simulator,
            None => return Err(Error::SimulatorNotFound(image_id.to_string())),
        };

        let image = repository.find_by_id(simulator.image_id()).await?;

        let image = match image {
            Some(image) => image,
            None => return Err(Error::ImageNotFound(simulator.image_id().to_string())),
        };

        let docker_container =
            DockerSimulator::create(docker.clone(), environment, simulator, &image).await?;

        let docker_simulator = docker_container.start(Some(tx.clone())).await?;

        image_id_to_running_docker_simulator.insert(image_id, docker_simulator);
    }

    Ok(image_id_to_running_docker_simulator)
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

async fn run_scenario(
    step_data: &[(&Step, &RunningDockerSimulator)],
    tx: Arc<UnboundedSender<ScenarioPlayingEvent>>,
) -> Result<(), Error> {
    for (step, running_docker_simulator) in step_data.iter() {
        trace!(
            "Command: {:?}, Arguments: {:?}",
            step.command,
            step.arguments
        );

        let command_result = running_docker_simulator
            .execute_command(&step.command.path, &step.arguments)
            .await;

        match command_result {
            Ok(response) => {
                tx.send(ScenarioPlayingEvent::StepPassed { message: response })
                    .ok();
            }
            Err(error) => {
                if let Error::SimulatorCommandFailed { message, status } = error {
                    return Err(Error::SimulatorCommandFailed { message, status });
                };
            }
        };

        trace!("Ran command {:?}", step.command);
    }

    Ok(())
}
