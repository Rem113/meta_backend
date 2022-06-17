use std::{convert::Infallible, sync::Arc};

use bollard::Docker;
use mongodb::Database;
use warp::Filter;

use crate::api::handlers::scenarios_handlers;
use crate::data::Repository;

pub fn scenarios_routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
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

    let run = common
        .and(warp::path("run"))
        .and(warp::path::param())
        .and(warp::path("environment"))
        .and(warp::path::param())
        .and(with_docker(docker))
        .and(warp::ws())
        .and_then(scenarios_handlers::run);

    list.or(create).or(run)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (Repository,), Error = Infallible> + Clone {
    warp::any().map(move || Repository::new(database.clone()))
}

fn with_docker(
    docker: Arc<Docker>,
) -> impl Filter<Extract = (Arc<Docker>,), Error = Infallible> + Clone {
    warp::any().map(move || docker.clone())
}
