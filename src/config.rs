use crate::error::GdeError;
use std::path::Path;
use serde_json::Value;

pub struct Config {
    content: Value,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self,GdeError> {
        let data = String::new();
        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(&data)?;

        Ok(Self {
            content: v
        })
    }

    pub fn get_value(&self, index:&str) -> Option<String> {
        if let Some(value) =self.content.get(index) {
            Some(value.to_string())
        } else {
            None
        }
    }
}