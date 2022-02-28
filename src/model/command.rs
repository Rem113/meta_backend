use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    name: String,
    description: String,
}

impl Command {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}
