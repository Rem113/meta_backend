use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::data::{Environment, Error};

pub struct EnvironmentRepository {
    environments: Collection<Environment>,
}

impl EnvironmentRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            environments: database.collection("Environments"),
        }
    }

    pub async fn list(&self) -> Result<Vec<Environment>, Error> {
        let cursor = self.environments.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
