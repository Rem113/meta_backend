use std::sync::Arc;

use bollard::{
    container::{Config, CreateContainerOptions},
    models::HostConfig,
    Docker,
};

use crate::data::{Environment, Image, Repository, Scenario, Simulator};

use super::error::Error;

pub struct DockerExecutor {
}

impl DockerExecutor {
    pub async fn run_scenario_in_environment(
        docker: Arc<Docker>,
        environment: &Environment,
        scenario: &Scenario,
        repository: Repository,
    ) -> Result<(), Error> {
        let steps = scenario.steps();

        for step in steps.iter() {
            let simulator = repository
                .find_by_id::<Simulator>(&step.simulator_id)
                .await?;

            let simulator = match simulator {
                Some(simulator) => simulator,
                None => return Err(Error::SimulatorNotFound(step.clone())),
            };

            let image = repository.find_by_id(simulator.image_id()).await?;

            let image = match image {
                Some(image) => image,
                None => return Err(Error::ImageNotFound(step.clone())),
            };

            create_container(&docker, environment, simulator, image).await?;
        }

        Ok(())
    }

}

async fn create_container(
    docker: &Arc<Docker>,
    environment: &Environment,
    simulator: Simulator,
    image: Image,
) -> Result<(), Error> {
    docker
        .create_container(
            Some(CreateContainerOptions {
                name: format!("{}-{}", environment.name(), simulator.name()),
            }),
            Config {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                image: Some(image.tag().as_meta()),
                host_config: Some(HostConfig {
                    publish_all_ports: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}