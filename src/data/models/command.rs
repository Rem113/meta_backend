use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub path: String,
}

impl From<Command> for mongodb::bson::Document {
    fn from(command: Command) -> Self {
        doc! {
            "name": command.name,
            "description": command.description,
            "path": command.path,
        }
    }
}