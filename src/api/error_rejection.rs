use crate::api::error::Error;
use crate::data;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorRejection {
    message: String,
}

impl warp::reject::Reject for ErrorRejection {}

impl ErrorRejection {
    pub fn from(error: &str) -> Self {
        Self {
            message: error.to_string(),
        }
    }

    pub fn reject(error: &str) -> warp::Rejection {
        warp::reject::custom(ErrorRejection::from(error))
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
