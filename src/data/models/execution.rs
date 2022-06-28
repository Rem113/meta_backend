use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::data::models::scenario_playing_event::ScenarioPlayingEvent;
use crate::data::repository::Document;

use super::serializers::serialize_option_object_id;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExecutionDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "scenarioId")]
    scenario_id: String,
    #[serde(rename = "environmentId")]
    environment_id: String,
    timestamp: DateTime<Local>,
    events: Vec<ScenarioPlayingEvent>,
}

impl From<Execution> for ExecutionDTO {
    fn from(execution: Execution) -> Self {
        Self {
            id: execution.id.as_ref().map(ToString::to_string),
            scenario_id: execution.scenario_id.to_string(),
            environment_id: execution.environment_id.to_string(),
            timestamp: execution.timestamp,
            events: execution.events,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Execution {
    #[serde(alias = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_object_id")]
    id: Option<ObjectId>,
    #[serde(rename = "scenarioId")]
    scenario_id: ObjectId,
    #[serde(rename = "environmentId")]
    environment_id: ObjectId,
    timestamp: DateTime<Local>,
    events: Vec<ScenarioPlayingEvent>,
}

impl Execution {
    pub fn new(
        scenario_id: ObjectId,
        environment_id: ObjectId,
        timestamp: DateTime<Local>,
        events: Vec<ScenarioPlayingEvent>,
    ) -> Execution {
        Self {
            id: None,
            scenario_id,
            environment_id,
            timestamp,
            events,
        }
    }
}

impl Document for Execution {
    fn collection_name() -> &'static str {
        "Executions"
    }

    fn with_id(self, id: ObjectId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}
