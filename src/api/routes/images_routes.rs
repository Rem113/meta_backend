use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::data::ImageRepository;

pub fn images_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("images").and(with_repository(database));

    common
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_handler)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (ImageRepository,), Error = Infallible> + Clone {
    warp::any().map(move || ImageRepository::new(database.clone().as_ref()))
}

async fn list_handler(repository: ImageRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let images = repository.list().await?;

    Ok(warp::reply::json(&images))
}
