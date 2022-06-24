use std::sync::Arc;

use bollard::container::{LogOutput, LogsOptions, RemoveContainerOptions};
use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    models::HostConfig,
    Docker,
};
use futures::stream::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crate::data::{Environment, Image, Simulator};
use crate::domain::scenario_playing_event::ScenarioPlayingEvent;

use super::{running_docker_simulator::RunningDockerSimulator, Error};

pub struct DockerSimulator {
    container_name: String,
    simulator_name: String,
    docker: Arc<Docker>,
}

impl DockerSimulator {
    pub async fn create(
        docker: Arc<Docker>,
        environment: &Environment,
        simulator: &Simulator,
        image: &Image,
    ) -> Result<DockerSimulator, Error> {
        let container_name = format!("{}-{}", environment.name(), simulator.name());

        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: &container_name,
                }),
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

        Ok(DockerSimulator {
            container_name,
            simulator_name: simulator.name().to_string(),
            docker,
        })
    }

    pub async fn start(
        self,
        tx: Option<Arc<UnboundedSender<ScenarioPlayingEvent>>>,
    ) -> Result<RunningDockerSimulator, Error> {
        self.docker
            .start_container(self.container_name(), None::<StartContainerOptions<String>>)
            .await?;

        match get_exposed_port_for_container(self.docker.clone(), self.container_name()).await {
            Ok(port) => {
                if let Some(sender) = tx {
                    self.attach_logs(sender);
                };

                Ok(RunningDockerSimulator::new(
                    self.container_name().to_owned(),
                    port,
                    self.docker,
                ))
            }
            Err(_) => {
                self.docker
                    .remove_container(
                        self.container_name(),
                        Some(RemoveContainerOptions {
                            force: true,
                            ..Default::default()
                        }),
                    )
                    .await
                    .ok();

                Err(Error::SimulatorNotReady(String::from(
                    "No port exposed by simulator",
                )))
            }
        }
    }

    fn attach_logs(&self, tx: Arc<UnboundedSender<ScenarioPlayingEvent>>) {
        let docker = self.docker.clone();
        let container_name = self.container_name().to_owned();
        let simulator_name = self.simulator_name().to_owned();

        tokio::spawn(async move {
            docker
                .logs(
                    &container_name,
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
                    match log {
                        LogOutput::StdOut { message } | LogOutput::Console { message } => {
                            tx.send(ScenarioPlayingEvent::LogReceived {
                                simulator_name: simulator_name.clone(),
                                message: String::from_utf8_lossy(message.as_ref()).into(),
                                is_error: false,
                            })
                        }
                        LogOutput::StdErr { message } => {
                            tx.send(ScenarioPlayingEvent::LogReceived {
                                simulator_name: simulator_name.clone(),
                                message: String::from_utf8_lossy(message.as_ref()).into(),
                                is_error: true,
                            })
                        }
                        _ => Ok(()),
                    }
                    .ok();
                })
                .await;
        });
    }

    pub fn container_name(&self) -> &String {
        &self.container_name
    }

    pub fn simulator_name(&self) -> &String {
        &self.simulator_name
    }
}

async fn get_exposed_port_for_container(
    docker: Arc<Docker>,
    container_name: &str,
) -> Result<u16, Error> {
    let container_inspect_response = docker.inspect_container(container_name, None).await?;

    let option_port = container_inspect_response
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

    match option_port {
        Some(port) => Ok(port),
        None => Err(Error::SimulatorNotFound(format!(
            "Could not find exposed port for simulator {}",
            container_name
        ))),
    }
}
