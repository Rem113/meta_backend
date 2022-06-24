use bollard::Docker;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use warp::hyper;

use crate::data::{EnvironmentDTO, Scenario, Simulator, SimulatorDTO};
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
) -> Result<warp::reply::Json, warp::Rejection> {
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

    Ok(warp::reply::json(&EnvironmentDTO::from(environment)))
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

pub async fn run_scenario_in_environment(
    repository: Repository,
    environment_id: String,
    scenario_id: String,
    docker: Arc<Docker>,
    web_socket: warp::ws::Ws,
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
