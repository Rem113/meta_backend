use crate::data::repository::Document;

use super::serializers::serialize_option_object_id;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Environment {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
}

impl Environment {
    pub fn new(name: String) -> Environment {
        Environment { id: None, name }
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
