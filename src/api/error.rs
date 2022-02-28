use crate::data;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DatabaseError(#[from] data::Error),
}
