use bollard::{
    container::{Config, CreateContainerOptions},
    Docker,
};

use crate::data::{Environment, Image, Scenario, Simulator};

#[allow(dead_code)]
pub async fn run_scenario(
    docker: &Docker,
    environment: &Environment,
    scenario: Scenario,
    simulators: Vec<Simulator>,
    images: Vec<Image>,
) {
    let steps = scenario.steps();

    let stuff_tuple: Vec<(_, _, _)> = steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            let simulator = simulators
                .iter()
                .find(|simulator| match simulator.id() {
                    Some(id) => id.eq(&step.simulator_id),
                    None => false,
                })
                .unwrap_or_else(|| panic!("Couldn't find simulator for step #{}", index + 1));

            (step, simulator)
        })
        .map(|(step, simulator)| {
            let image = images
                .iter()
                .find(|image| match image.id() {
                    Some(id) => id.eq(simulator.image_id()),
                    None => false,
                })
                .expect("Couldn't find image for simulator");

            (step, simulator, image)
        })
        .collect();

    let mut test = stuff_tuple.iter().map(|(_, simulator, image)| async move {
        run_simulator(docker, environment, simulator, image).await
    });

    for _ in test.by_ref() {}
}

async fn run_simulator(
    docker: &Docker,
    environment: &Environment,
    simulator: &Simulator,
    image: &Image,
) -> Result<(), Box<dyn std::error::Error>> {
    docker
        .create_container(
            Some(CreateContainerOptions {
                name: format!("{}-{}", environment.name(), simulator.name()),
            }),
            Config {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                image: Some(format!("meta/{}:{}", image.name(), image.version())),
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
