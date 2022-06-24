use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use warp::hyper;

use crate::data::{EnvironmentDTO, Simulator, SimulatorDTO};
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
