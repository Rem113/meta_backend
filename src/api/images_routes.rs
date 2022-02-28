use std::sync::Arc;

use mongodb::Database;
use warp::reply::Json;
use warp::{Filter, Rejection};

use crate::api::middlewares::with_database;
use crate::model::ImageRepository;

pub fn images_routes(
    db: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let common = warp::path("images").and(with_database(db.clone()));

    let list = common
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_handler);

    list
}

pub async fn list_handler(db: Arc<Database>) -> Result<Json, Rejection> {
    let images = ImageRepository::list(db).await?;

    Ok(warp::reply::json(&images))
}
