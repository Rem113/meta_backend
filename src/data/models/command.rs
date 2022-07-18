use mongodb::bson::{bson, doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub path: String,
}

impl From<Command> for mongodb::bson::Bson {
    fn from(command: Command) -> Self {
        bson! ({
            "name": command.name,
            "description": command.description,
            "path": command.path,
        })
    }
}
