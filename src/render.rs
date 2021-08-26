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
    output: String,
}

impl RenderOptions {
    pub fn new(
        preserve:bool,
        purge:bool,
        copy: Option<PathBuf>,
        format: Option<String>,
        input: Option<PathBuf>,
        output: Option<PathBuf>
    ) -> Self {
        let input = if let Some(content) = input {
            content
        } else {
            PathBuf::from("index.gddt")
        };

        let output = if let Some(content) = output {
            content
        } else {
            PathBuf::from("out.gddt")
        };
        Self { 
            preserve,
            purge,
            copy,
            format,
            input : input.to_str().unwrap().to_owned(),
            output : output.to_str().unwrap().to_owned(),
        }
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

        // Return vec
        Ok(args)
    }

    fn set_rad_sources(&self) -> Result<Vec<String>, GdeError> {
        // Standard macro definition file
        let mut sources = vec!( utils::std_path()?.to_str().unwrap().to_owned() );

        // Set lib definition file
        let mut lib_path = utils::lib_path()?;
        lib_path.push(format!("{}.r4f", self.target));
        sources.push(lib_path.to_str().unwrap().to_owned());

        // TODO
        // User defined macro files
        sources.push(self.options.input.clone());

        Ok(sources)
    }

    fn render_main(&self) -> Result<(), GdeError> {
        match self.target {
            "marp" =>{
                // Change name to out.md
                let mut new_path = utils::middle_file_path()?;
                new_path.set_extension("md");
                std::fs::rename(utils::middle_file_path()?, &new_path)?;

                marp::marp_render(
                    new_path,
                    self.options.format.clone(),
                    &Path::new(&self.options.output)
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
