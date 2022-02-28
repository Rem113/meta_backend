use futures::TryStreamExt;
use mongodb::Database;

use crate::model::{Environment, Error};

pub struct EnvironmentRepository {}

impl EnvironmentRepository {
    pub async fn list(database: &Database) -> Result<Vec<Environment>, Error> {
        let collection = database.collection("Environment");
        let cursor = collection.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
