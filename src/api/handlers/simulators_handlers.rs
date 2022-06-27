use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;
use warp::hyper;

use serde::Deserialize;

use crate::data::SimulatorDTO;
use crate::{
    api::error_rejection::ErrorRejection,
    data::{Environment, Image, Repository, Simulator},
};

pub async fn list(repository: Repository) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators = repository.list::<Simulator>().await?;

    Ok(warp::reply::json(
        &simulators
            .into_iter()
            .map(SimulatorDTO::from)
            .collect::<Vec<_>>(),
    ))
}

#[derive(Debug, Deserialize)]
pub struct CreateSimulatorData {
    pub name: String,
    pub environment_id: ObjectId,
    pub image_id: ObjectId,
    pub configuration: HashMap<String, String>,
}

pub async fn create(
    repository: Repository,
    simulator_data: CreateSimulatorData,
) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators_for_environment = repository
        .find::<Simulator>(doc! {"environmentId": simulator_data.environment_id})
        .await?;

    for other in &simulators_for_environment {
        if other.name() == simulator_data.name {
            return Err(ErrorRejection::reject(
                "A simulator with the same name exists in this environment",
                hyper::StatusCode::CONFLICT,
            ));
        }
    }

    let port = match simulators_for_environment
        .iter()
        .map(|simulator| simulator.port())
        .max()
    {
        Some(port) => port + 1,
        None => 30000,
    };

    let environment = repository
        .find_by_id::<Environment>(&simulator_data.environment_id)
        .await?;
    let image = repository
        .find_by_id::<Image>(&simulator_data.image_id)
        .await?;

    match (environment, image) {
        (Some(_), Some(_)) => {
            let simulator = Simulator::new(
                simulator_data.name,
                port,
                simulator_data.environment_id,
                simulator_data.image_id,
                simulator_data.configuration,
            );

            let simulator = repository.create(simulator).await?;

            Ok(warp::reply::json(&SimulatorDTO::from(simulator)))
        }
        (None, None) => Err(ErrorRejection::reject(
            "Environment and image not found",
            hyper::StatusCode::NOT_FOUND,
        )),
        (None, _) => Err(ErrorRejection::reject(
            "Environment not found",
            hyper::StatusCode::NOT_FOUND,
        )),
        (_, None) => Err(ErrorRejection::reject(
            "Image not found",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}
