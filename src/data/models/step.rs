use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Command;
use super::serializers::serialize_object_id;

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

impl From<Step> for mongodb::bson::Document {
    fn from(step: Step) -> Self {
        doc! {
            "imageId": step.image_id,
            "command": mongodb::bson::Document::from(step.command),
            "arguments": step.arguments.to_string(),
        }
    }
}