use std::path::PathBuf;
use std::process::Command;
use crate::error::GdeError;
use crate::utils;
use crate::renderer::*;
use crate::config::Config;

pub struct ExecOptions {
    // Option used by rad
    purge: bool,
    input: String,
    // Used by post process
    copy: Option<PathBuf>,
    // Used by renderer
    preserve: bool,
    format: Option<String>,
    out_file: Option<PathBuf>,
}

impl ExecOptions {
    pub fn new(
        preserve:bool,
        purge:bool,
        copy: Option<PathBuf>,
        format: Option<String>,
        input: Option<PathBuf>,
        output: Option<PathBuf>
    ) -> Result<Self,GdeError> {
        let input = if let Some(content) = input {
            content
        } else {
            utils::DEFAULT_ENTRY_PATH.to_owned()
        };

        Ok(Self { 
            preserve,
            purge,
            copy,
            format,
            input : input.to_str().unwrap().to_owned(),
            out_file: output,
        })
    }
}

pub struct Executor<'a> {
    renderer: &'a str,
    options : ExecOptions,
    config: Config,
}

impl<'a> Executor<'a> {
    pub fn new(renderer: &'a str, options: ExecOptions, config: Config) -> Self {
        Self { 
            renderer,
            options,
            config
        }
    }

    pub fn exec(&self) -> Result<(), GdeError> {
        self.preprocess()?;
        self.macro_expansion()?;
        self.render()?;
        self.postprocess()?;

        Ok(())
    }

    fn preprocess(&self) -> Result<(), GdeError> {
        std::env::set_var("GDE_MODULE", utils::renderer_path(self.renderer)?);
        Ok(())
    }

    fn macro_expansion(&self) -> Result<(), GdeError> {
        let args = self.set_rad_arguments()?;

        let output = Command::new("rad")
            .args(&args)
            .output()?;

        // Print out error
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        Ok(())
    }

    fn set_rad_arguments(&self) -> Result<Vec<String>, GdeError> {
        // Set include files
        let mut args = self.set_rad_sources()?;
        
        // Set other options
        // Purge option
        if self.options.purge {
            args.push("-p".to_owned());
        }

        // Set out file, or middle file
        let out_file = utils::middle_file_path()?
            .to_str()
            .unwrap()
            .to_owned();
        args.push("-o".to_owned());
        args.push(out_file);
        args.push("-g".to_owned());

        // Return vec
        Ok(args)
    }

    fn set_rad_sources(&self) -> Result<Vec<String>, GdeError> {
        // User defined macro
        let mut sources = vec!(self.options.input.clone());
        // Standard macro definition file
        sources.push("-m".to_owned());
        sources.push(format!("{}", utils::STD_MACRO_PATH.display()));
        // Set lib definition file
        sources.push(format!("{}", utils::module_path(self.renderer)?.display()));

        Ok(sources)
    }

    fn render(&self) -> Result<(), GdeError> {
        match self.renderer {
            "marp" =>{
                marp::render( &self.options.format, &self.options.out_file)?;
            }
            "mediawiki" => {
                mediawiki::render(&self.config)?;
            }
            "gdlogue" => {
                gdlogue::render(&self.options.format, &self.options.out_file)?;
            }
            "flowchartjs" => {
                flowchartjs::render(&self.options.out_file)?;
            }
            "flowchartgvz" => {
                flowchartgvz::render(&self.options.format,&self.options.out_file)?;
            }
            "webuibts" => {
                webuibts::render(&self.options.format,&self.options.out_file)?;
            }
            _ => eprintln!("No appropriate renderer was given"),
        }
        Ok(())
    }

    // Copy output file to designated file
    // Remove cached file
    fn postprocess(&self) -> Result<(), GdeError> {
        if let Some(path) = &self.options.copy {

        }
        if self.options.preserve {

        }
        Ok(())
    }
}
