mod db;
mod images;

pub use db::init_db;
pub use images::ImageRepository;
use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum ModelError {
    #[error("{0}")]
    InitializationError(#[from] mongodb::error::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Environment {
    pub name: String,
    pub simulators: Vec<Simulator>,
}

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    name: String,
    description: String,
}

impl Command {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulator {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    image_id: ObjectId,
    configuration: HashMap<String, String>,
}

impl Simulator {
    pub fn new(name: String, image_id: ObjectId, configuration: HashMap<String, String>) -> Self {
        Self {
            id: None,
            name,
            image_id,
            configuration,
        }
    }

    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image_id(&self) -> &ObjectId {
        &self.image_id
    }

    pub fn configuration(&self) -> &HashMap<String, String> {
        &self.configuration
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    pub simulator_id: String,
    pub command_id: String,
    pub arguments: Vec<String>,
}
