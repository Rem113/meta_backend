use std::convert::Infallible;
use std::sync::Arc;

use crate::api::handlers;
use mongodb::Database;
use warp::filters::body;
use warp::Filter;

use crate::data::ImageRepository;

pub fn images_routes(
    database: Arc<Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("images").and(with_repository(database));

    let list = common
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handlers::list_handler);

    let create = common
        .and(warp::post())
        .and(warp::path::end())
        .and(body::json())
        .and_then(handlers::create_handler)
        .recover(handlers::rejection::create_rejection_handler);

    list.or(create)
}

fn with_repository(
    database: Arc<Database>,
) -> impl Filter<Extract = (ImageRepository,), Error = Infallible> + Clone {
    warp::any().map(move || ImageRepository::new(database.clone().as_ref()))
}
