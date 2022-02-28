use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::routes::{environment_routes, images_routes};

mod error;
mod error_rejection;
mod middlewares;
mod routes;

pub fn routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    images_routes(database.clone()).or(environment_routes(database))
}
