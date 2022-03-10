#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DockerInit(String),
    #[error("{0}")]
    DatabaseInit(String),
}

impl From<bollard::errors::Error> for Error {
    fn from(error: bollard::errors::Error) -> Self {
        Error::DockerInit(error.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(error: mongodb::error::Error) -> Self {
        Error::DatabaseInit(error.to_string())
    }
}
