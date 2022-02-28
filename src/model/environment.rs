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
