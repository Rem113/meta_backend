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

    pub async fn create(&self, image: Image) -> Result<Image, Error> {
        let result = self.images.insert_one(image.clone(), None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(image.with_id(inserted_id))
    }
}
