use crate::api::error::Error;
use crate::data;

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

impl From<data::Error> for warp::Rejection {
    fn from(other: data::Error) -> Self {
        ErrorRejection::reject(&other.to_string())
    }
}
