use crate::model;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DatabaseError(#[from] model::Error),
}
