use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    simulators_id: Vec<ObjectId>,
}

impl Environment {
    pub fn new(name: String, simulators_id: Vec<ObjectId>) -> Environment {
        Environment {
            id: None,
            name,
            simulators_id,
        }
    }
}
