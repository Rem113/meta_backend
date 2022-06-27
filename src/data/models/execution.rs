use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::data::repository::Document;
use crate::domain::LogMessage;

use super::serializers::serialize_option_object_id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Execution {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    logs: Vec<LogMessage>,
}

impl Document for Execution {
    fn collection_name() -> &'static str {
        "Executions"
    }

    fn with_id(self, id: ObjectId) -> Self {
        todo!()
    }
}
