use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMessage {
    #[serde(rename = "simulatorName")]
    pub simulator_name: String,
    pub timestamp: DateTime<Local>,
    pub message: String,
    #[serde(rename = "isError")]
    pub is_error: bool,
}
