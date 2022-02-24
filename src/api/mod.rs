mod images;

use crate::model;

use crate::model::ModelError;
pub use images::images_routes;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DatabaseError(#[from] model::ModelError),
    #[error("{0}")]
    OtherError(String),
}

#[derive(Debug)]
pub struct WebServerError {
    pub message: String,
}

impl warp::reject::Reject for WebServerError {}

impl WebServerError {
    pub fn reject(error: &str) -> warp::Rejection {
        warp::reject::custom(WebServerError {
            message: error.to_string(),
        })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebServerError::reject(&other.to_string())
    }
}

impl From<model::ModelError> for warp::Rejection {
    fn from(other: ModelError) -> Self {
        WebServerError::reject(&other.to_string())
    }
}
