use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::middlewares;
use crate::model::SimulatorRepository;

pub fn simulators_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::reply::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("simulators").and(middlewares::with_database(database));

    common
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_handler)
}

async fn list_handler(database: Arc<Database>) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators = SimulatorRepository::list(database.as_ref()).await?;

    Ok(warp::reply::json(&simulators))
}
