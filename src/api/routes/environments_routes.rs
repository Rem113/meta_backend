use bollard::Docker;
use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::environments_handlers;
use crate::data::Repository;

pub fn environment_routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("environments").and(with_repository(database));

    let list = common
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(environments_handlers::list);

    let create = common
        .clone()
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(environments_handlers::create);

    let find_by_id = common
        .clone()
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(environments_handlers::find_by_id);

    let simulators_for_environment = common
        .clone()
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path("simulators"))
        .and(warp::path::end())
        .and_then(environments_handlers::simulators_for_environment);

    let run_scenario_in_environment = common
        .and(warp::path::param())
        .and(warp::path("scenarios"))
        .and(warp::path::param())
        .and(with_docker(docker))
        .and(warp::ws())
        .and_then(environments_handlers::run_scenario_in_environment);

    list.or(create)
        .or(find_by_id)
        .or(simulators_for_environment)
        .or(run_scenario_in_environment)
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
