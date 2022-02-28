use crate::api::error::Error;
use crate::model;

#[derive(Debug)]
pub struct ErrorRejection(String);

impl warp::reject::Reject for ErrorRejection {}

impl ErrorRejection {
    pub fn reject(error: &str) -> warp::Rejection {
        warp::reject::custom(ErrorRejection(error.to_string()))
    }
}

impl From<Error> for warp::Rejection {
    fn from(other: Error) -> Self {
        ErrorRejection::reject(&other.to_string())
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        ErrorRejection::reject(&other.to_string())
    }
}
