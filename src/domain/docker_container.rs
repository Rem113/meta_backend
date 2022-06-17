use std::sync::Arc;

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions, StopContainerOptions},
    models::HostConfig,
    Docker,
};
use tracing::trace;

use crate::data::{Environment, Image, Simulator};

use super::{docker_simulator::DockerSimulator, Error};

pub struct DockerContainer {
    name: String,
}

impl DockerContainer {
    pub async fn create(
        docker: Arc<Docker>,
        environment: &Environment,
        simulator: &Simulator,
        image: &Image,
    ) -> Result<DockerContainer, Error> {
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

        Ok(DockerContainer { name })
    }

    pub async fn start(self, docker: Arc<Docker>) -> Result<DockerSimulator, Error> {
        docker
            .start_container(self.name(), None::<StartContainerOptions<String>>)
            .await?;

        let port = get_exposed_port_for_container(docker, self.name()).await?;

        Ok(DockerSimulator::new(self.name().to_owned(), port))
    }

    pub async fn remove(docker_simulator: DockerSimulator, docker: Arc<Docker>) -> Result<(), Error> {
        docker
            .stop_container(docker_simulator.name(), None::<StopContainerOptions>)
            .await?;

        Ok(())
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
