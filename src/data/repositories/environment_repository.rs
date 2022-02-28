use futures::TryStreamExt;
use mongodb::Database;

use crate::data::{Environment, Error};

pub struct EnvironmentRepository {}

impl EnvironmentRepository {
    pub async fn list(database: &Database) -> Result<Vec<Environment>, Error> {
        let collection = database.collection("Environments");
        let cursor = collection.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
