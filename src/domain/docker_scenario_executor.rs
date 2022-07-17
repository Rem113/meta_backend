use std::time::SystemTime;
use std::{collections::HashMap, sync::Arc, time::Duration};

use bollard::Docker;
use chrono::DateTime;
use futures::{future::try_join_all, SinkExt};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{trace, warn};
use warp::ws::Message;

use crate::data::{Execution, ScenarioPlayingEvent};
use crate::{
    data::{Environment, Repository, Scenario, Simulator, Step},
    domain::{docker_simulator::DockerSimulator, running_docker_simulator::RunningDockerSimulator},
};

use super::error::DomainError;

pub struct DockerScenarioExecutor {}

impl DockerScenarioExecutor {
    pub async fn run_scenario_in_environment(
        docker: Arc<Docker>,
        environment: &Environment,
        scenario: &Scenario,
        repository: Repository,
        mut web_socket: warp::ws::WebSocket,
    ) -> Result<(), DomainError> {
        let steps = scenario.steps();

        let unique_images =
            steps
                .iter()
                .map(|step| step.image_id)
                .fold(Vec::new(), |mut accumulator, current| {
                    if accumulator.contains(&current) {
                        accumulator
                    } else {
                        accumulator.push(current);
                        accumulator
                    }
                });

        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        let image_id_to_running_simulator = instantiate_simulators(
            unique_images,
            repository.clone(),
            docker.clone(),
            environment,
            tx.clone(),
        )
        .await?;

        let running_simulators = image_id_to_running_simulator
            .values()
            .cloned()
            .collect::<Vec<_>>();

        wait_for_simulators_to_be_ready(running_simulators.clone(), tx.clone()).await?;

        let scenario_id = scenario.id().unwrap().to_owned();
        let environment_id = environment.id().unwrap().to_owned();

        tokio::spawn(async move {
            let mut events = Vec::new();

            while let Some(event) = rx.recv().await {
                events.push(event.clone());

                if let Err(err) = web_socket
                    .send(Message::text(
                        serde_json::to_string(&event).unwrap_or_default(),
                    ))
                    .await
                {
                    warn!("{:?}", err);
                }
            }

            repository
                .clone()
                .create::<Execution>(Execution::new(
                    scenario_id,
                    environment_id,
                    DateTime::from(SystemTime::now()),
                    events,
                ))
                .await
                .ok();
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
                DomainError::SimulatorCommandFailed {
                    step,
                    message,
                    status,
                } => tx
                    .send(ScenarioPlayingEvent::StepFailed {
                        step,
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
) -> Result<HashMap<ObjectId, RunningDockerSimulator>, DomainError> {
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
            None => return Err(DomainError::SimulatorNotFound(image_id.to_string())),
        };

        let image = repository.find_by_id(simulator.image_id()).await?;

        let image = match image {
            Some(image) => image,
            None => return Err(DomainError::ImageNotFound(simulator.image_id().to_string())),
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
    tx: Arc<UnboundedSender<ScenarioPlayingEvent>>,
) -> Result<(), DomainError> {
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

                Err(DomainError::SimulatorNotReady(format!(
                    "Simulator {} was not ready within 30 seconds",
                    running_simulator.name()
                )))
            })
        })
        .collect::<Vec<_>>();

    let result = try_join_all(ready_futures).await.map_err(|error| {
        DomainError::SimulatorNotReady(format!(
            "Error while waiting for simulators to be ready: {}",
            error
        ))
    })?;

    tx.send(ScenarioPlayingEvent::ScenarioStarting).ok();

    result.into_iter().collect()
}

async fn run_scenario(
    step_data: &[(&Step, &RunningDockerSimulator)],
    tx: Arc<UnboundedSender<ScenarioPlayingEvent>>,
) -> Result<(), DomainError> {
    for (i, (step, running_docker_simulator)) in step_data.iter().enumerate() {
        trace!(
            "Step #{}: Command: {:?}, Arguments: {:?}",
            i + 1,
            step.command,
            step.arguments
        );

        let command_result = running_docker_simulator
            .execute_command(i + 1, &step.command.path, &step.arguments)
            .await;

        match command_result {
            Ok(response) => {
                tx.send(ScenarioPlayingEvent::StepPassed {
                    step: i + 1,
                    message: response,
                })
                .ok();
            }
            Err(error) => {
                if matches!(error, DomainError::SimulatorCommandFailed { .. }) {
                    return Err(error);
                }
            }
        };

        trace!("Ran command {:?}", step.command);
    }

    Ok(())
}
