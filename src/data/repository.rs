use std::sync::Arc;

use crate::data::Error;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{de::DeserializeOwned, Serialize};
pub trait Document {
    fn collection_name() -> &'static str;
    fn with_id(self, id: ObjectId) -> Self;
}

pub struct Repository {
    database: Arc<Database>,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Self {
            database: Arc::clone(&self.database),
        }
    }
}

impl Repository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn list<T>(&self) -> Result<Vec<T>, Error>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        let cursor = collection.find(None, None).await?;

        cursor.try_collect().await.map_err(Into::into)
    }

    pub async fn create<T: Document>(&self, document: T) -> Result<T, Error>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection::<T>(T::collection_name());

        let result = collection.insert_one(&document, None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(document.with_id(inserted_id))
    }

    pub async fn find_by_id<T: Document>(&self, id: &ObjectId) -> Result<Option<T>, Error>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        collection
            .find_one(doc! {"_id": id}, None)
            .await
            .map_err(Into::into)
    }

    pub async fn find<T: Document>(
        &self,
        document: mongodb::bson::Document,
    ) -> Result<Vec<T>, Error>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        let cursor = collection.find(document, None).await?;

        cursor.try_collect().await.map_err(Into::into)
    }
}
