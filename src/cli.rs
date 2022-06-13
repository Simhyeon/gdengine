use crate::config::Config;
use crate::error::GdeError;
use crate::executor::{ExecOption, Executor};
use crate::init::Init;
use crate::models::GdeResult;
use crate::orchestrator::Orchestrator;
use crate::utils::{self, BUILD_PATH};
use clap::{App, Arg};
use std::path::Path;
use std::path::PathBuf;

pub enum Variant {
    Test,
    Run,
    None,
}

/// Struct to parse command line arguments and execute proper operations
pub struct Cli {
    variant: Variant,
}

impl Cli {
    pub fn new(variant: Variant) -> Self {
        Self { variant }
    }

    pub fn parse(&self) -> GdeResult<()> {
        let cli_args = Cli::get_arg_matches();
        self.parse_options(&cli_args)?;
        Ok(())
    }

    pub fn parse_options(&self, args: &clap::ArgMatches) -> GdeResult<()> {
        if let Some(action) = args.value_of("ACTION") {
            match action {
                "init" => {
                    // Create new files and directories
                    Init::new_gddt_file()?;
                    Init::new_rad_file()?;
                    Init::new_config_file()?;
                    Init::new_env_file()?;
                    Init::new_var_file()?;
                    Init::new_macro_file()?;
                    Init::new_directories()?;
                    // Git option
                    if args.is_present("git") {
                        Init::git_init()?;
                    }
                }
                "test" => {
                    let config = Config::from(&utils::CONFIG_PATH)?;
                    let target = args.value_of("target");
                    Orchestrator::test(&config, target)?;
                }
                "run" => {
                    let config = Config::from(&utils::CONFIG_PATH)?;
                    let target = args.value_of("target");
                    Orchestrator::run(&config, target)?;
                }
                "render" => {
                    // Set environment variables
                    Self::set_env_vars(args)?;

                    let variable_list = Self::set_variables(args)?;

                    if let Some(module) = args.value_of("module") {
                        // Set module
                        let render_option = self.parse_exec_options(args);

                        // Execute
                        Executor::new(module, render_option, variable_list)?.exec()?;
                    } else {
                        eprintln!("No proper render module was provided");
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

    fn set_env_vars(matches: &clap::ArgMatches) -> GdeResult<()> {
        if let Some(envs) = matches.value_of("env") {
            let envs = envs.split(',').collect::<Vec<&str>>();
            for env in envs {
                println!("Setting env: {}", env);
                std::env::set_var(env, "");
            }
        }

        if let Some(file) = matches.value_of("envfile") {
            let path = Path::new(file);
            // Path doesn't exist
            if !path.exists() {
                return Err(GdeError::NoSuchPath(file.to_owned()));
            }

            dotenv::from_path(path).expect("Failed to parse env file");
        }

        Ok(())
    }

    fn set_variables(matches: &clap::ArgMatches) -> GdeResult<Option<Vec<(String, String)>>> {
        use std::io::BufRead;

        if let Some(file) = matches.value_of("varfile") {
            let file = std::fs::File::open(file)?;
            let buf = std::io::BufReader::new(file);
            let variables: Result<Vec<(String, String)>, GdeError> = buf
                .lines()
                .map(|l| {
                    let line = l.expect("Failed to read line from variable file");
                    let splitted: Vec<&str> = line.splitn(2, '=').collect();
                    if splitted.len() < 2 {
                        return Err(GdeError::VarFileError(format!(
                            "\n{}\n is not a valid variable syntax",
                            line
                        )));
                    }
                    Ok((splitted[0].to_owned(), splitted[1].to_owned()))
                })
                .collect();
            Ok(Some(variables?))
        } else {
            Ok(None)
        }
    }

    pub fn get_string_matches(input: &str) -> clap::ArgMatches {
        let mut args = vec!["gde"];
        args.extend(input.split_whitespace());
        Self::args_builder().get_matches_from(args)
    }

    fn args_builder() -> clap::App<'static> {
        App::new("gde")
            .version("0.5.0")
            .author("Simon creek <simoncreek@tutanota.com>")
            .about("Gdengine is a document automation program.")
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::new("ACTION")
                    .help("An action to take")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::new("help")
                    .help("Display help message")
                    .short('h')
                    .long("help"),
            )
            .arg(
                Arg::new("preserve")
                    .help("Preserve temporary files")
                    .short('p')
                    .long("preserve"),
            )
            .arg(
                Arg::new("strict")
                    .help("Make strict mode")
                    .short('s')
                    .long("strict"),
            )
            .arg(
                Arg::new("input")
                    .help("Input file to process")
                    .short('i')
                    .long("input")
                    .takes_value(true),
            )
            .arg(
                Arg::new("output")
                    .help("Output file to save result")
                    .short('o')
                    .long("output")
                    .takes_value(true),
            )
            .arg(
                Arg::new("copy")
                    .help("Copy to directory")
                    .short('c')
                    .long("copy")
                    .takes_value(true),
            )
            .arg(
                Arg::new("module")
                    .help("Render module")
                    .short('m')
                    .long("module")
                    .takes_value(true),
            )
            .arg(
                Arg::new("format")
                    .help("Render format")
                    .short('f')
                    .long("format")
                    .takes_value(true),
            )
            .arg(
                Arg::new("test")
                    .help("Render yields extra information help process")
                    .long("test"),
            )
            .arg(Arg::new("diff").help("Show diff result").long("diff"))
            .arg(
                Arg::new("toc")
                    .help("Return for table of contents feature")
                    .long("toc"),
            )
            .arg(
                Arg::new("target")
                    .help("Script target to execute")
                    .short('t')
                    .long("target")
                    .takes_value(true),
            )
            .arg(Arg::new("log").help("Log macro invocations").long("log"))
            .arg(Arg::new("git").help("Init with git").long("git"))
            .arg(
                Arg::new("env")
                    .help("Set environment variables")
                    .short('e')
                    .long("env")
                    .takes_value(true),
            )
            .arg(
                Arg::new("envfile")
                    .help("Set environment variables from a file")
                    .short('E')
                    .long("envfile")
                    .takes_value(true),
            )
            .arg(
                Arg::new("varfile")
                    .help("Set static macros from varaible file")
                    .short('V')
                    .long("varfile")
                    .takes_value(true),
            )
    }

    fn parse_exec_options(&self, matches: &clap::ArgMatches) -> ExecOption {
        ExecOption::new(matches.value_of("input").map(PathBuf::from))
            .preserve(matches.is_present("preserve"))
            .strict(matches.is_present("strict"))
            .test(matches.is_present("test"))
            .diff(matches.is_present("diff"))
            .log(matches.is_present("log"))
            .toc(matches.is_present("toc"))
            .copy(matches.value_of("copy").map(PathBuf::from))
            .format(matches.value_of("format").map(|s| s.to_owned()))
            .out_file(matches.value_of("output").map(|s| {
                if let Variant::Test = self.variant {
                    BUILD_PATH.join(&format!("test_{}", s))
                } else {
                    PathBuf::from(s)
                }
            }))
    }
}
