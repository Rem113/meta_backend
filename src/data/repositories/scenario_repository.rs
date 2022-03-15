use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::data::{Error, Scenario};

pub struct ScenarioRepository {
    scenarios: Collection<Scenario>,
}

impl ScenarioRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            scenarios: database.collection("Scenarios"),
        }
    }

    pub async fn list(&self) -> Result<Vec<Scenario>, Error> {
        let cursor = self.scenarios.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
