use std::collections::HashMap;
use mongodb::bson::{doc, to_bson};

use crate::data::repository::Document;

use super::serializers::{serialize_object_id, serialize_option_object_id};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimulatorDTO {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    port: u16,
    #[serde(rename = "environmentId")]
    environment_id: String,
    #[serde(rename = "imageId")]
    image_id: String,
    configuration: HashMap<String, String>,
}

impl From<Simulator> for SimulatorDTO {
    fn from(simulator: Simulator) -> Self {
        Self {
            id: simulator.id.as_ref().map(ToString::to_string),
            name: simulator.name,
            port: simulator.port,
            environment_id: simulator.environment_id.to_string(),
            image_id: simulator.image_id.to_string(),
            configuration: simulator.configuration,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulator {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    port: u16,
    #[serde(serialize_with = "serialize_object_id")]
    #[serde(rename = "environmentId")]
    environment_id: ObjectId,
    #[serde(serialize_with = "serialize_object_id")]
    #[serde(rename = "imageId")]
    image_id: ObjectId,
    configuration: HashMap<String, String>,
}

impl Simulator {
    pub fn new(
        name: String,
        port: u16,
        environment_id: ObjectId,
        image_id: ObjectId,
        configuration: HashMap<String, String>,
    ) -> Self {
        Self {
            id: None,
            name,
            port,
            environment_id,
            image_id,
            configuration,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn port(&self) -> u16 {
        self.port
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

impl From<Simulator> for mongodb::bson::Document {
    fn from(simulator: Simulator) -> Self {
        doc! {
            "name": simulator.name,
            "port": simulator.port as u32,
            "environmentId": simulator.environment_id,
            "imageId": simulator.image_id,
            "configuration": to_bson(&simulator.configuration).unwrap(),
        }
    }
}