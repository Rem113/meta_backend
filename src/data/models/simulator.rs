use std::collections::HashMap;

use crate::data::repository::Document;

use super::serializers::{serialize_object_id, serialize_option_object_id};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulator {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    #[serde(serialize_with = "serialize_object_id")]
    environment_id: ObjectId,
    #[serde(serialize_with = "serialize_object_id")]
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn environment_id(&self) -> &ObjectId {
        &self.environment_id
    }

    pub fn image_id(&self) -> &ObjectId {
        &self.image_id
    }

    pub fn configuration(&self) -> &HashMap<String, String> {
        &self.configuration
    }
}

impl Document for Simulator {
    fn collection_name() -> &'static str {
        "Simulators"
    }

    fn with_id(self, id: ObjectId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}
