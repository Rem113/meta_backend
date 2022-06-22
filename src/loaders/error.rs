#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Docker(String),
    #[error("{0}")]
    Database(String),
}

impl From<bollard::errors::Error> for Error {
    fn from(error: bollard::errors::Error) -> Self {
        Error::Docker(error.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(error: mongodb::error::Error) -> Self {
        Error::Database(error.to_string())
    }
}

impl From<crate::domain::Error> for Error {
    fn from(error: crate::domain::Error) -> Self {
        match error {
            crate::domain::Error::Docker(error) => Error::Docker(error.to_string()),
            crate::domain::Error::Data(error) => Error::Database(error.to_string()),
            other => panic!("Unexpected error while initializing the app: {:?}", other),
        }
    }
}
