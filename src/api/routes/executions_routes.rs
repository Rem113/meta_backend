use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::{executions_handler, simulators_handlers};
use crate::data::Repository;

pub fn executions_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("executions").and(with_repository(database));

    let find_by_id = common
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(executions_handler::find_by_id);

    find_by_id
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (Repository,), Error = Infallible> + Clone {
    warp::any().map(move || Repository::new(database.clone()))
}
