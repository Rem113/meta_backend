use warp::hyper;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Docker(#[from] bollard::errors::Error),
    #[error("{0}")]
    Data(#[from] crate::data::Error),
    #[error("{0}")]
    SimulatorNotReady(String),
    #[error("Error {status}: {message}")]
    SimulatorCommandFailed {
        message: String,
        status: hyper::StatusCode,
    },
    #[error("Simulator not found. Simulator ID: {0:#?}")]
    SimulatorNotFound(String),
    #[error("Image not found. Image ID: {0:#?}")]
    ImageNotFound(String),
}
