use futures::TryStreamExt;
use mongodb::{bson::oid::ObjectId, Database};
use serde::{de::DeserializeOwned, Serialize};

use crate::data::Error;
pub trait Document {
    fn collection_name() -> &'static str;
    fn with_id(self, id: ObjectId) -> Self;
}

pub struct Repository<T> {
    collection: mongodb::Collection<T>,
}

impl<T: Sync + Send + Serialize + DeserializeOwned + Unpin + Document> Repository<T> {
    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection(T::collection_name()),
        }
    }

    pub async fn list(&self) -> Result<Vec<T>, Error> {
        let cursor = self.collection.find(None, None).await?;

        cursor.try_collect().await.map_err(Error::from)
    }

    pub async fn create(&self, document: T) -> Result<T, Error> {
        let result = self.collection.insert_one(&document, None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(document.with_id(inserted_id))
    }

    pub async fn find(&self, document: mongodb::bson::Document) -> Result<Vec<T>, Error> {
        let cursor = self.collection.find(Some(document), None).await?;

        cursor.try_collect().await.map_err(Error::from)
    }
}
