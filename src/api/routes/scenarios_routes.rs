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
        .and(warp::get())
        .and(warp::path::end())
        .and_then(scenarios_handlers::list);

    list
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (ScenarioRepository,), Error = Infallible> + Clone {
    warp::any().map(move || ScenarioRepository::new(database.clone().as_ref()))
}
