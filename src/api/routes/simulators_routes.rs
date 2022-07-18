use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::simulators_handlers;
use crate::data::Repository;

pub fn simulators_routes(
    database: Arc<Database>,
) -> impl Filter<Extract=(impl warp::reply::Reply, ), Error=warp::Rejection> + Clone {
    let common = warp::path("simulators").and(with_repository(database));

    let update = common
        .clone()
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(simulators_handlers::update);

    let remove = common
        .and(warp::delete())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(simulators_handlers::remove);

    update.or(remove)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract=(Repository, ), Error=Infallible> + Clone {
    warp::any().map(move || Repository::new(database.clone()))
}
