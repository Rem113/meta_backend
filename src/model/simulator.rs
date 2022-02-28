use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
