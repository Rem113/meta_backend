use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

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

        cursor.try_collect().await.map_err(Error::from)
    }

    pub async fn create(&self, environment: Environment) -> Result<Environment, Error> {
        let result = self.environments.insert_one(&environment, None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(environment.with_id(inserted_id))
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Environment>, Error> {
        self.environments
            .find_one(doc! {"name": name}, None)
            .await
            .map_err(Error::from)
    }
}
