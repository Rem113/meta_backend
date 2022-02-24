use crate::model::ImageRepository;
use mongodb::Database;
use std::convert::Infallible;
use std::sync::Arc;
use warp::reply::Json;
use warp::{Filter, Rejection};

pub fn images_routes(
    db: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let common = warp::path("images").and(with_db(db.clone()));

    let list = common
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_handler);

    list
}

fn with_db(
    db: Arc<Database>,
) -> impl Filter<Extract = (Arc<Database>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub async fn list_handler(db: Arc<Database>) -> Result<Json, Rejection> {
    let images = ImageRepository::list(db).await?;

    Ok(warp::reply::json(&images))
}
