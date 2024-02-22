use std::collections::HashMap;
use std::sync::Arc;

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    Docker,
    models::HostConfig,
};
use bollard::container::{LogOutput, LogsOptions, RemoveContainerOptions};
use bollard::models::PortBinding;
use chrono::Local;
use futures::stream::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crate::data::{Environment, Image, Simulator};
use crate::data::{LogMessage, ScenarioPlayingEvent};

use super::{DomainError, running_docker_simulator::RunningDockerSimulator};

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
    ) -> Result<DockerSimulator, DomainError> {
        let container_name = format!("{}-{}", environment.name(), simulator.name());

        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: &container_name,
                    platform: None,
                }),
                Config {
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    image: Some(image.tag().as_meta()),
                    host_config: Some(HostConfig {
                        port_bindings: Some(HashMap::from([(
                            String::from("3000/tcp"),
                            Some(vec![PortBinding {
                                host_ip: Some(String::from("127.0.0.1")),
                                host_port: Some(format!("{}", simulator.port())),
                            }]),
                        )])),
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
    ) -> Result<RunningDockerSimulator, DomainError> {
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

                Err(DomainError::SimulatorNotReady(String::from(
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
                    let (message, is_error) = match log {
                        LogOutput::StdIn { .. } => return,
                        LogOutput::StdOut { message } | LogOutput::Console { message } => {
                            (message, false)
                        }
                        LogOutput::StdErr { message } => (message, true),
                    };

                    // TODO: Stop crashing
                    let message_with_timestamp =
                        String::from_utf8_lossy(message.as_ref()).to_string();
                    let split_index = message_with_timestamp.find(' ').expect("No timestamp");
                    let (timestamp, message) = message_with_timestamp.split_at(split_index);
                    let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp)
                        .expect("Invalid timestamp")
                        .with_timezone(&Local);
                    let mut message = message[1..].to_owned();

                    if message.ends_with('\n') {
                        message.pop();
                    }

                    tx.send(ScenarioPlayingEvent::LogReceived {
                        log_message: LogMessage {
                            simulator_name: simulator_name.clone(),
                            timestamp,
                            message,
                            is_error,
                        },
                    })
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
) -> Result<u16, DomainError> {
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
        None => Err(DomainError::SimulatorNotFound(format!(
            "Could not find exposed port for simulator {}",
            container_name
        ))),
    }
}
