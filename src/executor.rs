use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::utils;
use crate::renderer::*;
use crate::config::Config;
use rad::{Processor, RadError};

pub struct ExecOptions {
    // Option used by rad
    purge: bool,
    input: String,
    test: bool,
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
        test:bool,
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
            test,
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
        if let Err(err) = self.macro_expansion() {
            eprintln!("{}", err);
            if self.options.test {
                eprintln!("{}", err);
                return Ok(());
            } else {
                return Err(GdeError::Raderror(err));
            }
        }
        let out_file = match self.render() {
            Result::Ok(value) => value,
            Err(err) => {
                // On test environment, it is fine to fail 
                if self.options.test {
                    eprintln!("{}", err);
                    return Ok(());
                } else {
                    return Err(err);
                }
            }
        };

        self.postprocess(out_file)?;

        Ok(())
    }

    fn preprocess(&self) -> Result<(), GdeError> {
        std::env::set_var("GDE_MODULE", utils::renderer_path(self.renderer)?);
        Ok(())
    }

    fn macro_expansion(&self) -> Result<(), RadError> {
        let mut processor = Processor::new()
            .purge(self.options.purge)
            .greedy(true)
            .write_to_file(Some(utils::middle_file_path().expect("Failed to get path")))?
            .custom_rules(Some(vec![
                    utils::STD_MACRO_PATH.to_owned(),
                    utils::module_path(self.renderer).expect("Failed to get path")
            ]))?.build();

        processor.from_file(Path::new(&self.options.input))?;

        if self.options.test {
             processor.print_result()?;
        }

        Ok(())
    }

    fn render(&self) -> Result<Option<PathBuf>, GdeError> {
        //println!("Render with \"{}\"", self.renderer);
        let out_file = match self.renderer {
            "marp" =>{
                marp::render( &self.options.format, &self.options.out_file)?
            }
            "mediawiki" => {
                mediawiki::render(&self.config)?
            }
            "pandoc" => {
                pandoc::render(&self.options.out_file)?
            }
            "gdlogue" => {
                gdlogue::render(&self.options.format, &self.options.out_file)?
            }
            "flowchartjs" => {
                flowchartjs::render(&self.options.out_file)?
            }
            "flowchartgvz" => {
                flowchartgvz::render(&self.options.format,&self.options.out_file)?
            }
            "webuibts" => {
                webuibts::render(&self.options.out_file)?
            }
            _ => {eprintln!("No appropriate renderer was given"); None}
        };
        
        Ok(out_file)
    }

    // Copy output file to designated file
    // Remove cached file
    fn postprocess(&self, final_file: Option<PathBuf>) -> Result<(), GdeError> {
        if let Some(final_file) = final_file {

            // Change middle name to Test_out.gddt
            if self.options.test{
                std::fs::rename(
                    utils::middle_file_path()?, 
                    utils::CACHE_PATH.join("Test_out.gddt")
                )?;
            } 

            if let Some(path) = &self.options.copy {
                if path.is_dir() {
                    std::fs::rename(&final_file, path.join(&final_file.file_name().unwrap()))?;
                } else {
                    std::fs::rename(final_file, path)?;
                }
            }
        }
        if !self.options.preserve {
            std::fs::remove_dir_all(utils::CACHE_PATH.to_owned())?;
            std::fs::create_dir(utils::CACHE_PATH.to_owned())?;
        }
        Ok(())
    }
}
