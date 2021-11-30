use crate::error::GdeError;
use crate::models::GdeResult;
use std::path::Path;
use serde_json::Value;

pub struct Config {
    content: Value,
}

impl Config {
    pub fn from(path: &Path) -> GdeResult<Self> {

        // Given config file doesn't exist
        if !path.exists() {
            return Err(GdeError::NotGdeDirectory);
        }

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(
            &String::from_utf8_lossy(&std::fs::read(path)?)
        )?;

        Ok(Self {
            content: v
        })
    }

    /// Create new config json string
    pub fn new_file() -> String {
        String::from(r#"{
    "run": {"marp": ["render -m marp"]},
    "test": {"marp": ["render -m marp --test --preserve"]}
}"#)
    }

    pub fn get_run_script(&self, name:Option<&str>) -> GdeResult<Option<Vec<Value>>> {
        self.get_script("run", name)
    }

    pub fn get_test_script(&self, name:Option<&str>) -> GdeResult<Option<Vec<Value>>> {
        self.get_script("test", name)
    }

    fn get_script(&self, script_type:&str, name: Option<&str>) -> GdeResult<Option<Vec<Value>>> {
        if let Some(value) =self.content.get(script_type) {
            let new_name :&str = if let None = name {
                let vec = value
                    .as_object()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<_>>();

                // Empty
                if vec.len() == 0 {
                    return Err(GdeError::ConfigError(format!("{} has empty content", script_type)));
                }

                // else return key of first element
                vec[0].0
            } else { name.unwrap() };

            if let Some(Value::Array(arr)) = value.get(new_name) {
                Ok(Some(arr.clone()))
            } else {
                eprintln!("No such target to execute : {}", new_name);
                Ok(None)
            }
        } else {
            Err(GdeError::ConfigError("No run element in config".to_owned()))
        }
    }
}
