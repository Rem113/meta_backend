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

    pub fn with_id(self, id: ObjectId) -> Image {
        Self {
            id: Some(id),
            name: self.name,
            version: self.version,
            commands: self.commands,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
