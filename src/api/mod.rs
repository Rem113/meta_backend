use bollard::Docker;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::routes::{environment_routes, images_routes, scenarios_routes, simulators_routes};

mod error;
mod error_rejection;
mod handlers;
mod routes;

pub fn routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    images_routes(database.clone(), docker)
        .or(environment_routes(database.clone()))
        .or(scenarios_routes(database.clone()))
        .or(simulators_routes(database))
}
