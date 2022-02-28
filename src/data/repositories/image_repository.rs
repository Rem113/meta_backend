use futures::TryStreamExt;
use mongodb::Database;

use crate::data::Error;
use crate::data::Image;

pub struct ImageRepository {}

impl ImageRepository {
    pub async fn list(database: &Database) -> Result<Vec<Image>, Error> {
        let images = database.collection("Images");
        let cursor = images.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
