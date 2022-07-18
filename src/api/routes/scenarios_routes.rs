use futures::TryFutureExt;
use std::{convert::Infallible, sync::Arc};

use mongodb::Database;
use tracing_subscriber::filter::FilterExt;
use warp::Filter;

use crate::api::handlers::scenarios_handlers;
use crate::data::Repository;

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
        .clone()
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(scenarios_handlers::create);

    let find_by_id = common
        .clone()
        .and(warp::get())
        .and(warp::path::param())
        .and_then(scenarios_handlers::find_by_id);

    let update = common
        .clone()
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(scenarios_handlers::update);

    let remove = common
        .and(warp::delete())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(scenarios_handlers::remove);

    list.or(create).or(find_by_id).or(update).or(remove)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (Repository,), Error = Infallible> + Clone {
    warp::any().map(move || Repository::new(database.clone()))
}
