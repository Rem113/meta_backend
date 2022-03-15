use crate::{
    api::error_rejection::ErrorRejection,
    data::{Environment, EnvironmentRepository},
};

pub async fn list(repository: EnvironmentRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let environments = repository.list().await?;

    Ok(warp::reply::json(&environments))
}

pub async fn create(
    repository: EnvironmentRepository,
    environment: Environment,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_environment = repository.find_by_name(environment.name()).await?;

    if already_existing_environment.is_some() {
        return Err(ErrorRejection::reject("Environment alreay exists"));
    }

    let environment = repository.create(environment).await?;

    Ok(warp::reply::json(&environment))
}
