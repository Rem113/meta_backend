#[derive(thiserror::Error, Debug)]
pub enum ModelError {
    #[error("{0}")]
    InitializationError(#[from] mongodb::error::Error),
}
