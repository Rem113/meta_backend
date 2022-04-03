use mongodb::bson::doc;

use crate::{
    api::error_rejection::ErrorRejection,
    data::{Environment, Repository},
};

pub async fn list(
    repository: Repository<Environment>,
) -> Result<warp::reply::Json, warp::Rejection> {
    let environments = repository.list().await?;

    Ok(warp::reply::json(&environments))
}

pub async fn create(
    repository: Repository<Environment>,
    environment: Environment,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_environment = repository.find(doc! {"name": environment.name()}).await?;

    if !already_existing_environment.is_empty() {
        return Err(ErrorRejection::reject("Environment alreay exists"));
    }

    let environment = repository.create(environment).await?;

    Ok(warp::reply::json(&environment))
}
