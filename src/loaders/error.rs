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

impl From<crate::domain::DomainError> for Error {
    fn from(error: crate::domain::DomainError) -> Self {
        match error {
            crate::domain::DomainError::Docker(error) => Error::Docker(error.to_string()),
            crate::domain::DomainError::Data(error) => Error::Database(error.to_string()),
            other => panic!("Unexpected error while initializing the app: {:?}", other),
        }
    }
}
