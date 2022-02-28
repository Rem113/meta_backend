use std::convert::Infallible;
use std::sync::Arc;

use mongodb::Database;
use warp::Filter;

pub fn with_database(
    database: Arc<Database>,
) -> impl Filter<Extract = (Arc<Database>,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}
