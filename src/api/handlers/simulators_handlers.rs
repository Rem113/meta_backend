use mongodb::bson::doc;
use warp::hyper;

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

pub async fn create(
    repository: Repository,
    simulator: Simulator,
) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators_for_environment = repository
        .find::<Simulator>(doc! {"environment_id": simulator.environment_id()})
        .await?;

    for other in simulators_for_environment {
        if other.name() == simulator.name() {
            return Err(ErrorRejection::reject(
                "A simulator with the same name exists in this environment",
                hyper::StatusCode::CONFLICT,
            ));
        }
    }

    let environment = repository
        .find_by_id::<Environment>(simulator.environment_id())
        .await?;
    let image = repository.find_by_id::<Image>(simulator.image_id()).await?;

    match (environment, image) {
        (Some(_), Some(_)) => {
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
