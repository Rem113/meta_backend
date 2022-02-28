use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    pub simulator_id: String,
    pub command_id: String,
    pub arguments: Vec<String>,
}
