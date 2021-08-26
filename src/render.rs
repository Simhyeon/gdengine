use std::path::{PathBuf, Path};
use std::process::Command;
use crate::error::GdeError;
use crate::utils;
use crate::renderer::marp;

pub struct RenderOptions {
    // Option used by rad
    purge: bool,
    input: String,
    // Used by post process
    copy: Option<PathBuf>,
    // Used by renderer
    preserve: bool,
    format: Option<String>,
    output: Option<PathBuf>,
}

impl RenderOptions {
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
            utils::default_entry_path()?
        };

        Ok(Self { 
            preserve,
            purge,
            copy,
            format,
            input : input.to_str().unwrap().to_owned(),
            output : output,
        })
    }
}

pub struct Renderer<'a> {
    target: &'a str,
    options : RenderOptions,
}

impl<'a> Renderer<'a> {
    pub fn new(target: &'a str, options: RenderOptions) -> Self {
        Self { 
            target,
            options,
        }
    }

    pub fn render(&self) -> Result<(), GdeError> {

        self.rad_process()?;
        self.render_main()?;
        self.postprocess()?;

        Ok(())
    }

    fn rad_process(&self) -> Result<(), GdeError> {
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
        sources.push(format!("{}", utils::std_path()?.display()));
        // Set lib definition file
        sources.push(format!("{}", utils::module_path(self.target)?.display()));

        Ok(sources)
    }

    fn render_main(&self) -> Result<(), GdeError> {
        match self.target {
            "marp" =>{
                // Change name to out.md
                let mut new_path = utils::middle_file_path()?;
                new_path.set_extension("md");
                std::fs::rename(utils::middle_file_path()?, &new_path)?;

                let format = if let Some(format) = &self.options.format {
                    format.to_owned()
                } else {
                    "html".to_owned()
                };

                let out_file = if let Some(name) = &self.options.output {
                    name.to_owned()
                } else {
                    utils::build_path()?.join(format!("out.{}", format)).to_owned()
                };

                marp::marp_render(
                    new_path,
                    &format,
                    &out_file
                )?;
            }
            "mw" => {

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
