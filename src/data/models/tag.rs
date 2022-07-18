use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub name: String,
    pub version: String,
}

impl Tag {
    pub fn as_meta(&self) -> String {
        format!("meta/{}:{}", self.name, self.version)
    }
}
