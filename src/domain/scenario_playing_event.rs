use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScenarioPlayingEvent {
    StepPassed {
        message: String,
    },
    StepFailed {
        message: String,
        status: u16,
    },
    LogReceived {
        #[serde(rename = "simulatorName")]
        simulator_name: String,
        message: String,
        #[serde(rename = "isError")]
        is_error: bool,
    },
}
