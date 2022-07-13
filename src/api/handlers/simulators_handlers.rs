use mongodb::bson::oid::ObjectId;
use warp::hyper;

use crate::api::error_rejection::ErrorRejection;
use crate::data::{Repository, Simulator, SimulatorDTO};

pub async fn update(
    repository: Repository,
    simulator_id: ObjectId,
    simulator: Simulator,
) -> Result<warp::reply::Json, warp::Rejection> {
    match repository.update::<Simulator>(&simulator_id, simulator.into()).await {
        Ok(simulator) => Ok(warp::reply::json(&SimulatorDTO::from(simulator))),
        Err(_) => Err(ErrorRejection::reject(
            "Failed to update simulator",
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}