use crate::domain::log_message::LogMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScenarioPlayingEvent {
    ScenarioStarting,
    StepPassed {
        step: usize,
        message: String,
    },
    StepFailed {
        step: usize,
        message: String,
        status: u16,
    },
    LogReceived {
        #[serde(rename = "logMessage")]
        log_message: LogMessage,
    },
}
