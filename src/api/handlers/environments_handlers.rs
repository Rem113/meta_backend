use mongodb::bson::doc;
use warp::hyper;

use crate::{
    api::error_rejection::ErrorRejection,
    data::{Environment, Repository},
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
        return Err(ErrorRejection::reject(
            "Environment alreay exists",
            hyper::StatusCode::CONFLICT,
        ));
    }

    let environment = repository.create(environment).await?;

    Ok(warp::reply::json(&environment))
}