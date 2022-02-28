use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::data::Error;
use crate::Simulator;

pub struct SimulatorRepository {
    simulators: Collection<Simulator>,
}

impl SimulatorRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            simulators: database.collection("Simulators"),
        }
    }

    pub async fn list(&self) -> Result<Vec<Simulator>, Error> {
        let cursor = self.simulators.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
