#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InitializationError(#[from] mongodb::error::Error),
}
