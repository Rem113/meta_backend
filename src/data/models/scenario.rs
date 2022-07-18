use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::data::{repository::Document, Step, StepDTO};

use super::serializers::serialize_option_object_id;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScenarioDTO {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    description: String,
    steps: Vec<StepDTO>,
}

impl From<Scenario> for ScenarioDTO {
    fn from(scenario: Scenario) -> Self {
        Self {
            id: scenario.id.as_ref().map(ToString::to_string),
            name: scenario.name,
            description: scenario.description,
            steps: scenario.steps.into_iter().map(StepDTO::from).collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    name: String,
    description: String,
    steps: Vec<Step>,
}

impl Scenario {
    pub fn new(name: String, description: String, steps: Vec<Step>) -> Self {
        Scenario {
            id: None,
            name,
            description,
            steps,
        }
    }

    pub fn id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }
}

impl Document for Scenario {
    fn collection_name() -> &'static str {
        "Scenarios"
    }

    fn with_id(self, id: ObjectId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}

impl From<Scenario> for mongodb::bson::Document {
    fn from(scenario: Scenario) -> Self {
        doc! {
            "name": scenario.name,
            "description": scenario.description,
            "steps": scenario.steps.into_iter().map(mongodb::bson::Bson::from).collect::<Vec<_>>(),
        }
    }
}
