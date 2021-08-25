use clap::clap_app;
use crate::error::GdeError;
use std::path::PathBuf;
use crate::init::Init;
use crate::render::{Renderer, RenderOptions};

/// Struct to parse command line arguments and execute proper operations
pub struct Cli{}

impl Cli {
    pub fn parse() -> Result<(), GdeError>{
        let cli_args = Cli::args_builder();
        Cli::parse_options(&cli_args)?;
        Ok(())
    }

    fn parse_options(args: &clap::ArgMatches) -> Result<(), GdeError> {
        if let Some(action) = args.value_of("ACTION") {
            match action {
                "init" => {
                    // Create new files and directories
                    Init::new_gddt_file()?;
                    Init::new_macro_file()?;
                    Init::new_directories()?;
                    // Git option
                    if args.is_present("git") {
                        Init::git_init()?;
                    }
                },
                "render" => {
                    if let Some(module) = args.value_of("module") {
                        // Set module
                        let render_option = Cli::parse_render_options(args)?;

                        // Render 
                        Renderer::new(
                            module,
                            render_option
                        ).render()?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    // TODO Add stream or file type option for main usage
    fn args_builder() -> clap::ArgMatches {
        clap_app!(Gde =>
            (version: "0.1.0")
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
        ).get_matches()
    }

    fn parse_render_options(matches: &clap::ArgMatches) -> Result<RenderOptions, GdeError> {
        Ok(RenderOptions::new(
                matches.is_present("preserve"),
                matches.is_present("purge"),
                matches.value_of("copy").map(|s| PathBuf::from(s)),
                matches.value_of("format").map(|s| s.to_owned()),
                matches.value_of("input").map(|s| PathBuf::from(s)),
                matches.value_of("output").map(|s| PathBuf::from(s)),
        ))
    }
}
