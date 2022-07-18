use std::{convert::Infallible, sync::Arc};

use bollard::Docker;
use mongodb::Database;
use warp::Filter;

use crate::api::routes::{environments_routes, executions_routes, images_routes, scenarios_routes, simulators_routes};

use self::error_rejection::ErrorRejection;

mod error;
mod error_rejection;
mod handlers;
mod routes;

pub fn routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
) -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    images_routes(Arc::clone(&database), Arc::clone(&docker))
        .or(environments_routes(Arc::clone(&database), docker))
        .or(scenarios_routes(Arc::clone(&database)))
        .or(executions_routes(Arc::clone(&database)))
        .or(simulators_routes(database))
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
