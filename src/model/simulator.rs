use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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
}
