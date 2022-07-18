use std::convert::Infallible;
use std::sync::Arc;

use bollard::Docker;
use mongodb::Database;
use tokio::sync::Mutex;
use warp::Filter;

use crate::api::handlers::environments_handlers;
use crate::data::Repository;

pub fn environments_routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
) -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
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

    let find_simulator_by_id = common
        .clone()
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path("simulators"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(environments_handlers::find_simulator_by_id);

    let execution_mutex = Arc::new(Mutex::new(()));

    let run_scenario_in_environment = common
        .clone()
        .and(warp::path::param())
        .and(warp::path("scenarios"))
        .and(warp::path::param())
        .and(with_docker(docker))
        .and(warp::ws())
        .and(with_mutex(execution_mutex))
        .and_then(environments_handlers::run_scenario_in_environment);

    let executions_for_scenario_in_environment = common
        .clone()
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path("scenarios"))
        .and(warp::path::param())
        .and(warp::path("executions"))
        .and_then(environments_handlers::executions_for_scenario_in_environment);

    let add_simulator_for_environment = common
        .and(warp::post())
        .and(warp::path::param())
        .and(warp::path("simulators"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(environments_handlers::add_simulator_for_environment);

    list.or(create)
        .or(find_by_id)
        .or(simulators_for_environment)
        .or(find_simulator_by_id)
        .or(run_scenario_in_environment)
        .or(executions_for_scenario_in_environment)
        .or(add_simulator_for_environment)
}

fn with_mutex(
    mutex: Arc<Mutex<()>>
) -> impl Filter<Extract=(Arc<Mutex<()>>, ), Error=Infallible> + Clone {
    warp::any().map(move || Arc::clone(&mutex))
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract=(Repository, ), Error=Infallible> + Clone {
    warp::any().map(move || Repository::new(database.clone()))
}

fn with_docker(
    docker: Arc<Docker>,
) -> impl Filter<Extract=(Arc<Docker>, ), Error=Infallible> + Clone {
    warp::any().map(move || docker.clone())
}
