use serde::{Deserialize, Serialize};

use crate::model::Step;

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}
