use mongodb::bson::doc;

use crate::{
    api::error_rejection::ErrorRejection,
    data::{Repository, Scenario},
};

pub async fn list(repository: Repository<Scenario>) -> Result<warp::reply::Json, warp::Rejection> {
    let scenarios = repository.list().await?;

    Ok(warp::reply::json(&scenarios))
}

pub async fn create(
    repository: Repository<Scenario>,
    scenario: Scenario,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_scenario = repository.find(doc! {"name": scenario.name() }).await?;

    if !already_existing_scenario.is_empty() {
        return Err(ErrorRejection::reject("Scenario already exists"));
    }

    let scenario = repository.create(scenario).await?;

    Ok(warp::reply::json(&scenario))
}
