use mongodb::bson::oid::ObjectId;
use warp::hyper;

use crate::api::error_rejection::ErrorRejection;
use crate::data::{Repository, Simulator, SimulatorDTO};

pub async fn update(
    repository: Repository,
    simulator_id: ObjectId,
    simulator: Simulator,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository
        .update::<Simulator>(&simulator_id, simulator.into())
        .await
    {
        Ok(simulator) => Ok(warp::reply::json(&SimulatorDTO::from(simulator))),
        Err(_) => Err(ErrorRejection::reject(
            "Failed to update simulator",
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn remove(
    repository: Repository,
    simulator_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository.remove::<Simulator>(&simulator_id).await {
        Ok(simulator) => Ok(warp::reply::json(&SimulatorDTO::from(simulator))),
        Err(_) => Err(ErrorRejection::reject(
            "Failed to remove simulator",
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
