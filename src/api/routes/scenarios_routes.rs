use std::{convert::Infallible, sync::Arc};

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::scenarios_handlers;
use crate::data::Repository;

pub fn scenario_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("scenarios").and(with_repository(database));

    let list = common
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(scenarios_handlers::list);

    let create = common
        .clone()
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(scenarios_handlers::create);

    let find_by_id = common
        .and(warp::get())
        .and(warp::path::param())
        .and_then(scenarios_handlers::find_by_id);

    list.or(create).or(find_by_id)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (Repository,), Error = Infallible> + Clone {
    warp::any().map(move || Repository::new(database.clone()))
}
