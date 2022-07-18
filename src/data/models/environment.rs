use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::data::repository::Document;

use super::serializers::serialize_option_object_id;

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentDTO {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    description: String,
}

impl From<Environment> for EnvironmentDTO {
    fn from(environment: Environment) -> Self {
        Self {
            id: environment.id.as_ref().map(ToString::to_string),
            name: environment.name,
            description: environment.description,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Environment {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    description: String,
}

impl Environment {
    pub fn new(name: String, description: String) -> Environment {
        Environment {
            id: None,
            name,
            description,
        }
    }

    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Document for Environment {
    fn collection_name() -> &'static str {
        "Environments"
    }

    fn with_id(self, id: ObjectId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}
