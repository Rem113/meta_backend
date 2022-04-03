use crate::data::repository::Document;

use super::serializers::serialize_option_object_id;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    version: String,
    commands: Vec<Command>,
}

impl Image {
    pub fn new(name: String, version: String, commands: Vec<Command>) -> Self {
        Self {
            id: None,
            name,
            version,
            commands,
        }
    }

    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Document for Image {
    fn collection_name() -> &'static str {
        "Images"
    }

    fn with_id(self, id: ObjectId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}
