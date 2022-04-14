use crate::data::Step;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Docker(#[from] bollard::errors::Error),
    #[error("{0}")]
    Data(#[from] crate::data::Error),
    #[error("Simulator not found. Step: {0:#?}")]
    SimulatorNotFound(Step),
    #[error("Image not found. Step: {0:#?}")]
    ImageNotFound(Step),
}