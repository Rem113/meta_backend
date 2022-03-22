use crate::{
    api::error_rejection::ErrorRejection,
    data::{Scenario, ScenarioRepository},
};

pub async fn list(repository: ScenarioRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let scenarios = repository.list().await?;

    Ok(warp::reply::json(&scenarios))
}

pub async fn create(
    repository: ScenarioRepository,
    scenario: Scenario,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_scenario = repository.find_by_name(scenario.name()).await?;

    if already_existing_scenario.is_some() {
        return Err(ErrorRejection::reject("Scenario already exists"));
    }

    let scenario = repository.create(scenario).await?;

    Ok(warp::reply::json(&scenario))
}
