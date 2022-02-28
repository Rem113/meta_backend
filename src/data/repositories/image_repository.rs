use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::data::Error;
use crate::data::Image;

pub struct ImageRepository {
    images: Collection<Image>,
}

impl ImageRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            images: database.collection("Images"),
        }
    }

    pub async fn list(&self) -> Result<Vec<Image>, Error> {
        let cursor = self.images.find(None, None).await?;

        let result = cursor.try_collect().await?;

        Ok(result)
    }
}
