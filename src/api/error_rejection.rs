use warp::hyper;

use crate::api::error::ApiError;
use crate::data::error::DataError;

#[derive(Debug)]
pub struct ErrorRejection {
    message: String,
    status: hyper::StatusCode,
}

impl warp::reject::Reject for ErrorRejection {}

impl ErrorRejection {
    pub fn from(error: &str, status: hyper::StatusCode) -> Self {
        Self {
            message: String::from(error),
            status,
        }
    }

    pub fn reject(error: &str, status: hyper::StatusCode) -> warp::Rejection {
        warp::reject::custom(ErrorRejection::from(error, status))
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn status(&self) -> hyper::StatusCode {
        self.status
    }
}

impl From<ApiError> for warp::Rejection {
    fn from(other: ApiError) -> Self {
        ErrorRejection::reject(&other.to_string(), hyper::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<DataError> for warp::Rejection {
    fn from(other: DataError) -> Self {
        ErrorRejection::reject(&other.to_string(), hyper::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
