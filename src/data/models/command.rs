use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub path: String,
}
