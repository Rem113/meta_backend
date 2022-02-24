use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::Database;

use crate::model::ModelError;
use crate::Image;

pub struct ImageRepository {}

impl ImageRepository {
    pub async fn list(db: Arc<Database>) -> Result<Vec<Image>, ModelError> {
        let images = db.collection("Images");
        let cursor = images.find(None, None).await?;

        let result: Vec<Image> = cursor.try_collect().await?;

        Ok(result)
    }
}
