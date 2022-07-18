use mongodb::bson::oid::ObjectId;
use mongodb::bson::{bson, doc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::serializers::serialize_object_id;
use super::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StepDTO {
    #[serde(rename = "imageId")]
    image_id: String,
    command: Command,
    arguments: Value,
}

impl From<Step> for StepDTO {
    fn from(step: Step) -> Self {
        Self {
            image_id: step.image_id.to_string(),
            command: step.command,
            arguments: step.arguments,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Step {
    #[serde(serialize_with = "serialize_object_id")]
    #[serde(rename = "imageId")]
    pub image_id: ObjectId,
    pub command: Command,
    pub arguments: Value,
}

impl From<Step> for mongodb::bson::Bson {
    fn from(step: Step) -> Self {
        bson! ({
            "imageId": step.image_id,
            "command": step.command,
            "arguments": mongodb::bson::to_bson(&step.arguments).expect("Failed to serialize arguments"),
        })
    }
}
