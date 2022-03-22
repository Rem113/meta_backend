use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::serializers::serialize_option_object_id;
use crate::data::Step;

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    description: String,
    steps: Vec<Step>,
}

impl Scenario {
    pub fn new(name: String, description: String, steps: Vec<Step>) -> Self {
        Scenario {
            id: None,
            name,
            description,
            steps,
        }
    }

    pub fn with_id(self, id: ObjectId) -> Self {
        Scenario {
            id: Some(id),
            ..self
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
