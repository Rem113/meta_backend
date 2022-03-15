use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::environment_handlers;
use crate::data::EnvironmentRepository;

pub fn environment_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("environments").and(with_repository(database));

    let list = common
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(environment_handlers::list);

    let create = common
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(environment_handlers::create);

    list.or(create)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (EnvironmentRepository,), Error = Infallible> + Clone {
    warp::any().map(move || EnvironmentRepository::new(database.clone().as_ref()))
}
