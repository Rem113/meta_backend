use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::data::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn with_id(&self, id: ObjectId) -> Image {
        Self {
            id: Some(id),
            name: self.name.clone(),
            version: self.version.clone(),
            commands: self.commands.clone(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
