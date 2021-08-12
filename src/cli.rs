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
        Cli::parse_subcommands(&cli_args)?;
        Ok(())
    }
    fn parse_subcommands(args: &clap::ArgMatches) -> Result<(), GdeError> {
        Cli::subcommand_init(args)?;
        Cli::subcommand_repr(args)?;
        Ok(())
    }

    // TODO Add stream or file type option for main usage
    fn args_builder() -> clap::ArgMatches {
        clap_app!(Gde =>
            (version: "0.0.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Gdengine is a document automation program.")
            (@setting ArgRequiredElseHelp)
            (@arg help: -h --help "Display help message")
            (@arg preserve: -p --preserve "Preserve temporary files")
            (@arg input: -n --input +takes_value "Input file to process")
            (@arg output: -o --output +takes_value "Output file to yield")
            (@arg copy: -c --copy +takes_value "Copy to directory")
            (@arg module: -m --module +takes_value "Copy to directory")
            (@subcommand init =>
                (about: "Initialize a directory")
                (@arg git: -g --git "Initialize with git directory")
            )
            (@subcommand repr =>
                (about: "Render representation format")
                (@arg format: ... -f --format +takes_value "Representation format (html|pdf|pptx)")
            )
        ).get_matches()
    }

    fn subcommand_init(matches: &clap::ArgMatches) -> Result<(), GdeError> {
        if let Some(sub_match) = matches.subcommand_matches("init") {
            // Create new files and directories
            Init::new_gddt_file()?;
            Init::new_macro_file()?;
            Init::new_directories()?;
            // Git option
            if sub_match.is_present("git") {
                Init::git_init()?;
            }
        } 
        Ok(())
    }

    fn subcommand_repr(matches: &clap::ArgMatches) -> Result<(), GdeError> {
        // Set module
        let module = matches.value_of("module");
        let render_option = Cli::parse_render_options(matches)?;

        if let Some(sub_match) = matches.subcommand_matches("repr") {
            // Set default module if not existent
            let module = if let None = module { "marp" } else { module.unwrap() };

            // Render specific option
            // Set format
            let mut format: Option<String> = None;
            if let Some(value) = sub_match.value_of("format") {
                format.replace(value.to_owned());
            }

            // Render 
            Renderer::new(
                "repr", 
                module, 
                render_option
            ).render()?;

        } 

        Ok(())
    }

    fn parse_render_options(matches: &clap::ArgMatches) -> Result<RenderOptions, GdeError> {
        Ok(RenderOptions::new(
                matches.is_present("preserve"),
                matches.value_of("copy").map(|str| PathBuf::from(str)),
                matches.value_of("format").map(|str| str.to_owned()),
                matches.value_of("input").map(|str| PathBuf::from(str)),
                matches.value_of("output").map(|str| PathBuf::from(str)),
        ))
    }
}
