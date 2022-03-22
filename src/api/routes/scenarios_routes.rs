use std::{convert::Infallible, sync::Arc};

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::scenarios_handlers;
use crate::data::ScenarioRepository;

pub fn scenarios_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("scenarios").and(with_repository(database));

    let list = common
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(scenarios_handlers::list);

    let create = common
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(scenarios_handlers::create);

    list.or(create)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (ScenarioRepository,), Error = Infallible> + Clone {
    warp::any().map(move || ScenarioRepository::new(database.clone().as_ref()))
}
