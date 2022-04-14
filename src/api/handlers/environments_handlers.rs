use std::sync::Arc;

use bollard::Docker;
use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    api::error_rejection::ErrorRejection,
    data::{Environment, Repository, Scenario}, domain::DockerExecutor,
};

pub async fn list(repository: Repository) -> Result<warp::reply::Json, warp::Rejection> {
    let environments = repository.list::<Environment>().await?;

    Ok(warp::reply::json(&environments))
}

pub async fn create(
    repository: Repository,
    environment: Environment,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_environment = repository
        .find::<Environment>(doc! {"name": environment.name()})
        .await?;

    if !already_existing_environment.is_empty() {
        return Err(ErrorRejection::reject("Environment alreay exists"));
    }

    let environment = repository.create(environment).await?;

    Ok(warp::reply::json(&environment))
}

pub async fn run(
    repository: Repository,
    docker: Arc<Docker>,
    environment_id: ObjectId,
    scenario_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    let environment = repository
        .find_by_id::<Environment>(&environment_id)
        .await?
        .unwrap();

    let scenario = repository
        .find_by_id::<Scenario>(&scenario_id)
        .await?
        .unwrap();

    DockerExecutor::run_scenario_in_environment(docker, &environment, &scenario, repository).await.map_err(|error| ErrorRejection::reject(&error.to_string()))?;

    Ok(warp::reply::json(&(environment, scenario)))
}
