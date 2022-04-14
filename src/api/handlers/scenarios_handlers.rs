use mongodb::bson::doc;

use crate::{
    api::error_rejection::ErrorRejection,
    data::{Repository, Scenario},
};

pub async fn list(repository: Repository) -> Result<warp::reply::Json, warp::Rejection> {
    let scenarios = repository.list::<Scenario>().await?;

    Ok(warp::reply::json(&scenarios))
}

pub async fn create(
    repository: Repository,
    scenario: Scenario,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_scenario = repository
        .find::<Scenario>(doc! {"name": scenario.name() })
        .await?;

    if !already_existing_scenario.is_empty() {
        return Err(ErrorRejection::reject("Scenario already exists"));
    }

    let scenario = repository.create(scenario).await?;

    Ok(warp::reply::json(&scenario))
}
