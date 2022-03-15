use std::collections::HashMap;

use super::serializers::serialize_option_object_id;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulator {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    environment_id: ObjectId,
    image_id: ObjectId,
    configuration: HashMap<String, String>,
}

impl Simulator {
    pub fn new(
        name: String,
        environment_id: ObjectId,
        image_id: ObjectId,
        configuration: HashMap<String, String>,
    ) -> Self {
        Self {
            id: None,
            name,
            environment_id,
            image_id,
            configuration,
        }
    }

    pub fn with_id(self, id: ObjectId) -> Simulator {
        Self {
            id: Some(id),
            name: self.name,
            environment_id: self.environment_id,
            image_id: self.image_id,
            configuration: self.configuration,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn environment_id(&self) -> &ObjectId {
        &self.environment_id
    }
}
