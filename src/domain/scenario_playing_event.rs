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
        #[serde(rename = "simulatorName")]
        simulator_name: String,
        message: String,
        #[serde(rename = "isError")]
        is_error: bool,
    },
}
