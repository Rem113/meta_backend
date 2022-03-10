use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::simulators_handlers;
use crate::data::SimulatorRepository;

pub fn simulators_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("simulators").and(with_repository(database));

    common
        .and(warp::get())
        .and(warp::path::end())
        .and_then(simulators_handlers::list)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (SimulatorRepository,), Error = Infallible> + Clone {
    warp::any().map(move || SimulatorRepository::new(database.clone().as_ref()))
}
