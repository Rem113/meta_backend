use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::middlewares;
use crate::data::ImageRepository;

pub fn images_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("images").and(middlewares::with_database(database));

    common
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_handler)
}

pub async fn list_handler(database: Arc<Database>) -> Result<warp::reply::Json, warp::Rejection> {
    let images = ImageRepository::list(database.as_ref()).await?;

    Ok(warp::reply::json(&images))
}
