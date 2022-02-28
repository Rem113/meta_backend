use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use images_routes::images_routes;

mod error;
mod error_rejection;
mod images_routes;
mod middlewares;

pub fn routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    images_routes(database.clone())
}
