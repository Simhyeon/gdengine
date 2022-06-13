use crate::cli::{Cli, Variant};
use crate::config::Config;
use crate::models::GdeResult;
use serde_json::Value;

pub struct Orchestrator;

impl Orchestrator {
    pub fn run(config: &Config, name: Option<&str>) -> GdeResult<()> {
        let values = config.get_run_script(name)?.unwrap_or_default();
        for value in values {
            if let Value::String(content) = value {
                println!("Running : {:?}", content);
                let matches = Cli::get_string_matches(&content);
                Cli::new(Variant::Run).parse_options(&matches)?;
            }
        }
        Ok(())
    }
    pub fn test(config: &Config, name: Option<&str>) -> GdeResult<()> {
        let values = config.get_test_script(name)?.unwrap_or_default();
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
