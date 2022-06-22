use std::sync::Arc;

use crate::data::{Environment, Image, Simulator};
use bollard::container::{LogOutput, LogsOptions, RemoveContainerOptions};
use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    models::HostConfig,
    Docker,
};
use futures::stream::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use super::{running_docker_simulator::RunningDockerSimulator, Error};

pub struct DockerSimulator {
    name: String,
    docker: Arc<Docker>,
}

impl DockerSimulator {
    pub async fn create(
        docker: Arc<Docker>,
        environment: &Environment,
        simulator: &Simulator,
        image: &Image,
    ) -> Result<DockerSimulator, Error> {
        let name = format!("{}-{}", environment.name(), simulator.name());

        docker
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

        Ok(DockerSimulator { name, docker })
    }

    pub async fn start(
        self,
        tx: Option<Arc<UnboundedSender<LogOutput>>>,
    ) -> Result<RunningDockerSimulator, Error> {
        self.docker
            .start_container(self.name(), None::<StartContainerOptions<String>>)
            .await?;

        match get_exposed_port_for_container(self.docker.clone(), self.name()).await {
            Ok(port) => {
                if let Some(sender) = tx {
                    self.attach_logs(sender);
                };

                Ok(RunningDockerSimulator::new(
                    self.name().to_owned(),
                    port,
                    self.docker,
                ))
            }
            Err(_) => {
                self.docker
                    .remove_container(
                        self.name(),
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

    fn attach_logs(&self, tx: Arc<UnboundedSender<LogOutput>>) {
        let docker = self.docker.clone();
        let name = self.name().to_owned();

        tokio::spawn(async move {
            docker
                .logs(
                    &name,
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

    pub fn name(&self) -> &String {
        &self.name
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
