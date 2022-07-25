use std::collections::HashMap;
use std::sync::Arc;

use bollard::Docker;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use tokio::sync::Mutex;
use warp::hyper;

use crate::data::{
    EnvironmentDTO, Execution, ExecutionDTO, Image, Scenario, Simulator, SimulatorDTO,
};
use crate::domain::DockerScenarioExecutor;
use crate::{
    api::error_rejection::ErrorRejection,
    data::{Environment, Repository},
};

pub async fn list(repository: Repository) -> Result<warp::reply::Json, warp::Rejection> {
    let environments = repository.list::<Environment>().await?;

    Ok(warp::reply::json(
        &environments
            .into_iter()
            .map(EnvironmentDTO::from)
            .collect::<Vec<_>>(),
    ))
}

pub async fn create(
    repository: Repository,
    environment: Environment,
) -> Result<impl warp::reply::Reply, warp::Rejection> {
    let already_existing_environment = repository
        .find::<Environment>(doc! {"name": environment.name()})
        .await?;

    if !already_existing_environment.is_empty() {
        return Err(ErrorRejection::reject(
            "Environment already exists",
            hyper::StatusCode::CONFLICT,
        ));
    }

    let environment = repository.create(environment).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&EnvironmentDTO::from(environment)),
        hyper::StatusCode::CREATED,
    ))
}

pub async fn find_by_id(
    repository: Repository,
    environment_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository
        .find_by_id::<Environment>(&environment_id)
        .await?
    {
        Some(environment) => Ok(warp::reply::json(&EnvironmentDTO::from(environment))),
        None => Err(ErrorRejection::reject(
            "Could not find environment",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn simulators_for_environment(
    repository: Repository,
    environment_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators = repository
        .find::<Simulator>(doc! { "environmentId": environment_id })
        .await?;

    Ok(warp::reply::json(
        &simulators
            .into_iter()
            .map(SimulatorDTO::from)
            .collect::<Vec<_>>(),
    ))
}

pub async fn find_simulator_by_id(
    repository: Repository,
    environment_id: ObjectId,
    simulator_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    let simulator = repository
        .find_one::<Simulator>(doc! { "_id": simulator_id, "environmentId": environment_id })
        .await?;

    match simulator {
        Some(simulator) => Ok(warp::reply::json(&SimulatorDTO::from(simulator))),
        None => Err(ErrorRejection::reject(
            "Could not find simulator",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn run_scenario_in_environment(
    repository: Repository,
    environment_id: String,
    scenario_id: String,
    docker: Arc<Docker>,
    web_socket: warp::ws::Ws,
    mutex: Arc<Mutex<()>>,
) -> Result<impl warp::reply::Reply, warp::Rejection> {
    let scenario_id = ObjectId::parse_str(&scenario_id).expect("Invalid scenario id");
    let environment_id = ObjectId::parse_str(&environment_id).expect("Invalid environment id");

    let environment = repository
        .find_by_id::<Environment>(&environment_id)
        .await?
        .expect("Environment not found");

    let scenario = repository
        .find_by_id::<Scenario>(&scenario_id)
        .await?
        .expect("Scenario not found");

    Ok(web_socket.on_upgrade(|web_socket| async move {
        // Prevents concurrent executions
        let _guard = mutex.lock().await;

        DockerScenarioExecutor::run_scenario_in_environment(
            docker,
            &environment,
            &scenario,
            repository,
            web_socket,
        )
        .await
        .ok();
    }))
}

pub async fn executions_for_scenario_in_environment(
    repository: Repository,
    environment_id: ObjectId,
    scenario_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    let executions = repository
        .find::<Execution>(doc! { "scenarioId": scenario_id, "environmentId": environment_id })
        .await?;

    Ok(warp::reply::json(
        &executions
            .into_iter()
            .map(ExecutionDTO::from)
            .collect::<Vec<_>>(),
    ))
}

#[derive(Debug, Deserialize)]
pub struct CreateSimulatorData {
    pub name: String,
    #[serde(rename = "imageId")]
    pub image_id: ObjectId,
    pub configuration: HashMap<String, String>,
}

pub async fn add_simulator_for_environment(
    repository: Repository,
    environment_id: ObjectId,
    simulator_data: CreateSimulatorData,
) -> Result<impl warp::reply::Reply, warp::Rejection> {
    let simulators_for_environment = repository
        .find::<Simulator>(doc! {"environmentId": environment_id})
        .await?;

    for other in &simulators_for_environment {
        if other.name() == simulator_data.name {
            return Err(ErrorRejection::reject(
                "A simulator with the same name exists in this environment",
                hyper::StatusCode::CONFLICT,
            ));
        }
    }

    let port = match simulators_for_environment
        .iter()
        .map(|simulator| simulator.port())
        .max()
    {
        Some(port) => port + 1,
        None => 30000,
    };

    let environment = repository
        .find_by_id::<Environment>(&environment_id)
        .await?;
    let image = repository
        .find_by_id::<Image>(&simulator_data.image_id)
        .await?;

    match (environment, image) {
        (Some(_), Some(_)) => {
            let simulator = Simulator::new(
                simulator_data.name,
                port,
                environment_id,
                simulator_data.image_id,
                simulator_data.configuration,
            );

            let simulator = repository.create(simulator).await?;

            Ok(warp::reply::with_status(
                warp::reply::json(&SimulatorDTO::from(simulator)),
                hyper::StatusCode::CREATED,
            ))
        }
        (None, None) => Err(ErrorRejection::reject(
            "Environment and image not found",
            hyper::StatusCode::NOT_FOUND,
        )),
        (None, _) => Err(ErrorRejection::reject(
            "Environment not found",
            hyper::StatusCode::NOT_FOUND,
        )),
        (_, None) => Err(ErrorRejection::reject(
            "Image not found",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}
