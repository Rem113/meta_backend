use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::serializers::serialize_object_id;
use super::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Step {
    #[serde(serialize_with = "serialize_object_id")]
    pub simulator_id: ObjectId,
    pub command: Command,
    pub arguments: Value,
}
