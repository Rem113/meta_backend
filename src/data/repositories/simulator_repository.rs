use futures::TryStreamExt;
use mongodb::Database;

use crate::data::Error;
use crate::Simulator;

pub struct SimulatorRepository {}

impl SimulatorRepository {
    pub async fn list(database: &Database) -> Result<Vec<Simulator>, Error> {
        let simulators = database.collection("Simulators");
        let cursor = simulators.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
