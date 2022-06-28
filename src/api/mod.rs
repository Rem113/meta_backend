use std::{convert::Infallible, sync::Arc};

use bollard::Docker;
use mongodb::Database;
use warp::Filter;

use crate::api::routes::{environment_routes, image_routes, scenario_routes, simulator_routes};

use self::error_rejection::ErrorRejection;

mod error;
mod error_rejection;
mod handlers;
mod routes;

pub fn routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    image_routes(database.clone(), docker.clone())
        .or(environment_routes(database.clone(), docker))
        .or(scenario_routes(database.clone()))
        .or(simulator_routes(database))
}

pub async fn rejection_handler(
    rejection: warp::Rejection,
) -> Result<impl warp::reply::Reply, Infallible> {
    let (message, status) = match rejection.find::<ErrorRejection>() {
        Some(error) => (error.message(), error.status()),
        None => (
            "Unknown error",
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&message),
        status,
    ))
}
