use crate::error::GdeError;
use crate::config::Config;
use crate::cli::{Cli, Variant};
use serde_json::Value;

pub struct Orchestrator;

impl Orchestrator {
    pub fn run(config: &Config) -> Result<(), GdeError> {
        let values = config.get_value_as_array("run").unwrap_or(vec![]);
        for value in values {
            if let Value::String(content) = value {
                println!("Running : {:?}", content);
                let matches = Cli::get_string_matches(&content);
                Cli::new(Variant::Run).parse_options(&matches)?;
            }
        }
        Ok(())
    }
    pub fn test(config: &Config) -> Result<(), GdeError> {
        let values = config.get_value_as_array("run").unwrap_or(vec![]);
        for value in values {
            if let Value::String(content) = value {
                println!("Running test : {:?}", content);
                let matches = Cli::get_string_matches(&content);
                Cli::new(Variant::Test).parse_options(&matches)?;
            }
        }
        Ok(())
    }
}
