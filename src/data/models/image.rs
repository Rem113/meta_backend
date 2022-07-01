use crate::data::repository::Document;

use super::{serializers::serialize_option_object_id, Tag};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageDTO {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    description: String,
    tag: Tag,
    commands: Vec<Command>,
}

impl From<Image> for ImageDTO {
    fn from(image: Image) -> Self {
        Self {
            id: image.id.as_ref().map(ToString::to_string),
            description: image.description,
            tag: image.tag,
            commands: image.commands,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    description: String,
    tag: Tag,
    commands: Vec<Command>,
}

impl Image {
    pub fn new(description: String, tag: Tag, commands: Vec<Command>) -> Self {
        Self {
            id: None,
            description,
            tag,
            commands,
        }
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }
}

impl Document for Image {
    fn collection_name() -> &'static str {
        "Images"
    }

    fn with_id(self, id: ObjectId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}
