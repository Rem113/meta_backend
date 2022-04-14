use crate::data::repository::Document;

use super::{serializers::serialize_option_object_id, Tag};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    tag: Tag,
    commands: Vec<Command>,
}

impl Image {
    pub fn new(tag: Tag, commands: Vec<Command>) -> Self {
        Self {
            id: None,
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
