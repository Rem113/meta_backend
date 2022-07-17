use warp::hyper;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("{0}")]
    Docker(#[from] bollard::errors::Error),
    #[error("{0}")]
    Data(#[from] crate::data::DataError),
    #[error("{0}")]
    SimulatorNotReady(String),
    #[error("Error {status}: {message}")]
    SimulatorCommandFailed {
        step: usize,
        message: String,
        status: hyper::StatusCode,
    },
    #[error("Simulator not found. Simulator ID: {0:#?}")]
    SimulatorNotFound(String),
    #[error("Image not found. Image ID: {0:#?}")]
    ImageNotFound(String),
}
