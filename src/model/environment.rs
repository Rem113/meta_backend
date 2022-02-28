use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::model::Simulator;

#[derive(Debug, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    simulators: Vec<Simulator>,
}

impl Environment {
    pub fn new(name: String, simulators: Vec<Simulator>) -> Self {
        Environment {
            id: None,
            name,
            simulators,
        }
    }

    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn simulators(&self) -> &Vec<Simulator> {
        &self.simulators
    }
}
