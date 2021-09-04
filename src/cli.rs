use clap::clap_app;
use crate::error::GdeError;
use crate::orchestrator::Orchestrator;
use std::path::PathBuf;
use crate::init::Init;
use crate::utils;
use crate::config::Config;
use crate::executor::{Executor, ExecOptions};

pub enum Variant {
    Test,
    Run,
    None,
}

/// Struct to parse command line arguments and execute proper operations
pub struct Cli{
    variant: Variant,
}

impl Cli {
    pub fn new(variant: Variant) -> Self {
        Self {
            variant,
        }
    }

    pub fn parse(&self) -> Result<(), GdeError>{
        let cli_args = Cli::get_arg_matches();
        self.parse_options(&cli_args)?;
        Ok(())
    }

    pub fn parse_options(&self, args: &clap::ArgMatches) -> Result<(), GdeError> {
        if let Some(action) = args.value_of("ACTION") {
            match action {
                "init" => {
                    // Create new files and directories
                    Init::new_gddt_file()?;
                    Init::new_config_file()?;
                    Init::new_macro_file()?;
                    Init::new_directories()?;
                    // Git option
                    if args.is_present("git") {
                        Init::git_init()?;
                    }
                },
                "test" => {
                    let config = if let Ok(config) = Config::from(&utils::CONFIG_PATH) { config
                    } else { return Err(GdeError::NotGdeDirectory); };
                    Orchestrator::test(&config)?;
                }
                "run" => {
                    let config = if let Ok(config) = Config::from(&utils::CONFIG_PATH) { config
                    } else { return Err(GdeError::NotGdeDirectory); };
                    Orchestrator::run(&config)?;
                }
                "render" => {
                    if let Some(module) = args.value_of("module") {
                        // Set module
                        let render_option = self.parse_exec_options(args)?;
                        let config = if let Ok(config) = Config::from(&utils::CONFIG_PATH) { config
                        } else { return Err(GdeError::NotGdeDirectory); };

                        // Execute 
                        Executor::new(
                            module,
                            render_option,
                            config
                        ).exec()?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn get_arg_matches() -> clap::ArgMatches {
        Self::args_builder().get_matches()
    }

    pub fn get_string_matches(args: &str) -> clap::ArgMatches {
        Self::args_builder().get_matches_from(args.split(' '))
    }

    // TODO Add stream or file type option for main usage
    fn args_builder() -> clap::App<'static> {
        clap_app!(Gde =>
            (version: "0.2.0")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Gdengine is a document automation program.")
            (@setting ArgRequiredElseHelp)
            (@arg ACTION: "An action to take")
            (@arg help: -h --help "Display help message")
            (@arg preserve: -p --preserve "Preserve temporary files")
            (@arg purge: -P --purge "Purge unused macro invocations")
            (@arg input: -n --input +takes_value "Input file to process")
            (@arg output: -o --output +takes_value "Output file to yield")
            (@arg copy: -c --copy +takes_value "Copy to directory")
            (@arg module: -m --module +takes_value "Render module")
            (@arg format: -f --format +takes_value "Render format")
        )
    }

    fn parse_exec_options(&self,matches: &clap::ArgMatches) -> Result<ExecOptions, GdeError> {
        Ok(ExecOptions::new(
                matches.is_present("preserve"),
                matches.is_present("purge"),
                matches.value_of("copy").map(|s| PathBuf::from(s)),
                matches.value_of("format").map(|s| s.to_owned()),
                matches.value_of("input").map(|s| PathBuf::from(s) ),
                matches.value_of("output").map(|s| if let Variant::Test = self.variant { PathBuf::from("TEST").join(s) } else { PathBuf::from(s) }),
        )?)
    }
}
