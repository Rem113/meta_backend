use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::data::DataError;

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

    pub async fn list<T>(&self) -> Result<Vec<T>, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        let cursor = collection.find(None, None).await?;

        cursor.try_collect().await.map_err(Into::into)
    }

    pub async fn create<T>(&self, document: T) -> Result<T, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection::<T>(T::collection_name());

        let result = collection.insert_one(&document, None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(document.with_id(inserted_id))
    }

    pub async fn find_by_id<T>(&self, id: &ObjectId) -> Result<Option<T>, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        collection
            .find_one(doc! {"_id": id}, None)
            .await
            .map_err(Into::into)
    }

    pub async fn find_one<T>(
        &self,
        document: mongodb::bson::Document,
    ) -> Result<Option<T>, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        collection
            .find_one(document, None)
            .await
            .map_err(Into::into)
    }

    pub async fn find<T>(&self, document: mongodb::bson::Document) -> Result<Vec<T>, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        let cursor = collection.find(document, None).await?;

        cursor.try_collect().await.map_err(Into::into)
    }

    pub async fn update<T>(
        &self,
        id: &ObjectId,
        document: mongodb::bson::Document,
    ) -> Result<T, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        let result = collection
            .find_one_and_update(doc! { "_id" : id }, doc! { "$set" : document }, None)
            .await?;

        result.ok_or(DataError::NotFound)
    }

    pub async fn remove<T>(&self, id: &ObjectId) -> Result<T, DataError>
    where
        T: Document + Unpin + Send + Sync + Serialize + DeserializeOwned,
    {
        let collection = self.database.collection(T::collection_name());

        let result = collection
            .find_one_and_delete(doc! { "_id" : id }, None)
            .await?;

        result.ok_or(DataError::NotFound)
    }
}
