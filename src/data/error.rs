#[derive(thiserror::Error, Debug)]
pub enum DataError {
    #[error("{0}")]
    InitializationError(#[from] mongodb::error::Error),
    #[error("Not found")]
    NotFound,
}
