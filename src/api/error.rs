use crate::data;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    DatabaseError(#[from] data::DataError),
}
