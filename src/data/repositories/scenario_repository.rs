use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

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

        cursor.try_collect().await.map_err(Error::from)
    }

    pub async fn create(&self, scenario: Scenario) -> Result<Scenario, Error> {
        let result = self.scenarios.insert_one(&scenario, None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(scenario.with_id(inserted_id))
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Scenario>, Error> {
        let filter = doc! {"name": name};

        self.scenarios
            .find_one(filter, None)
            .await
            .map_err(Error::from)
    }
}
