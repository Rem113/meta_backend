use chrono::format::Item::Error;
use mongodb::bson::{doc, oid::ObjectId};
use warp::hyper;

use crate::data::ScenarioDTO;
use crate::{
    api::error_rejection::ErrorRejection,
    data::{Repository, Scenario},
};

pub async fn list(repository: Repository) -> Result<warp::reply::Json, warp::Rejection> {
    let scenarios = repository.list::<Scenario>().await?;

    Ok(warp::reply::json(
        &scenarios
            .into_iter()
            .map(ScenarioDTO::from)
            .collect::<Vec<_>>(),
    ))
}

pub async fn create(
    repository: Repository,
    scenario: Scenario,
) -> Result<warp::reply::Json, warp::Rejection> {
    let already_existing_scenario = repository
        .find::<Scenario>(doc! {"name": scenario.name() })
        .await?;

    if !already_existing_scenario.is_empty() {
        return Err(ErrorRejection::reject(
            "Scenario already exists",
            hyper::StatusCode::CONFLICT,
        ));
    }

    let scenario = repository.create(scenario).await?;

    Ok(warp::reply::json(&ScenarioDTO::from(scenario)))
}

pub async fn find_by_id(
    repository: Repository,
    scenario_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository.find_by_id::<Scenario>(&scenario_id).await? {
        Some(scenario) => Ok(warp::reply::json(&ScenarioDTO::from(scenario))),
        None => Err(ErrorRejection::reject(
            "Could not find scenario",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn update(
    repository: Repository,
    scenario_id: ObjectId,
    scenario: Scenario,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository
        .update::<Scenario>(&scenario_id, scenario.into())
        .await
    {
        Ok(scenario) => Ok(warp::reply::json(&ScenarioDTO::from(scenario))),
        Err(_) => Err(ErrorRejection::reject(
            "Could not find scenario",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn remove(
    repository: Repository,
    scenario_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository.remove::<Scenario>(&scenario_id).await {
        Ok(scenario) => Ok(warp::reply::json(&ScenarioDTO::from(scenario))),
        Err(_) => Err(ErrorRejection::reject(
            "Could not find scenario",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}
