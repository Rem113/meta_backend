use bollard::Docker;
use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

use crate::api::handlers::images_handlers;
use crate::data::Repository;

pub fn images_routes(
    database: Arc<Database>,
    docker: Arc<Docker>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = warp::path("images").and(with_repository(database));

    let list = common
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(images_handlers::list);

    let create = common
        .and(warp::post())
        .and(warp::path::end())
        .and(with_docker(docker))
        .and(warp::filters::multipart::form())
        .and_then(images_handlers::create)
        .recover(images_handlers::rejection::create);

    list.or(create)
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
