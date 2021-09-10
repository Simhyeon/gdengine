use crate::error::GdeError;
use std::path::Path;
use serde_json::Value;

pub struct Config {
    content: Value,
}

impl Config {
    pub fn from(path: &Path) -> Result<Self,GdeError> {
        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(
            &String::from_utf8_lossy(&std::fs::read(path)?)
        )?;

        Ok(Self {
            content: v
        })
    }

    pub fn new_file() -> String {
        String::from(r#"{
    env: {},
    run: [],
    test: [""]
}"#)
    }

    pub fn get_value_as_array(&self, name:&str) -> Option<Vec<Value>> {
        if let Some(value) =self.content.get(name) {
            if let Value::Array(arr) = value {
                Some(arr.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_env_string(&self, index:&str) -> Option<String> {
        if let Some(value) =self.content.get("env")?.get(index) {
            if let serde_json::Value::String(content) = value {
                return Some(content.to_owned());
            }
        }
        return None;
    }
}
