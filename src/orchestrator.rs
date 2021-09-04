use crate::error::GdeError;
use crate::config::Config;
use crate::cli::{Cli, Variant};

pub struct Orchestrator;

impl Orchestrator {
    pub fn run(config: &Config) -> Result<(), GdeError> {
        let values = config.get_value_as_array("run").unwrap_or(vec![]);
        for value in values {
            println!("Running : {}", value);
            let matches = Cli::get_string_matches(&value.to_string());
            Cli::new(Variant::Run).parse_options(&matches)?;
        }
        Ok(())
    }
    pub fn test(config: &Config) -> Result<(), GdeError> {
        let values = config.get_value_as_array("run").unwrap_or(vec![]);
        for value in values {
            println!("Running test: {}", value);
            let matches = Cli::get_string_matches(&value.to_string());
            Cli::new(Variant::Test).parse_options(&matches)?;
        }
        Ok(())
    }
}
