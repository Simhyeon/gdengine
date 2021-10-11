use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::utils;
use crate::renderer::*;
use rad::{Processor, RadError, AuthType};

pub struct Executor {
    render_type: RenderType,
    options : ExecOptions,
}

impl Executor {
    pub fn new(render_type: &str, options: ExecOptions) -> Result<Self,GdeError> {
        Ok(Self { 
            render_type : RenderType::from(render_type)?,
            options,
        })
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
        std::env::set_var("GDE_MODULE", utils::renderer_path(self.render_type.to_string())?);
        Ok(())
    }

    fn macro_expansion(&self) -> Result<(), RadError> {
        let mut processor = Processor::new()
            .purge(true)
            .greedy(true)
            .lenient(!self.options.strict)
            .unix_new_line(true)
            .allow(Some(vec!(AuthType::ENV, AuthType::FIN, AuthType::FOUT, AuthType::CMD)))
            .write_to_file(Some(utils::middle_file_path().expect("Failed to get path")))?
            .custom_rules(Some(vec![
                    utils::STD_MACRO_PATH.to_owned(),
                    utils::module_path(self.render_type.to_string()).expect("Failed to get path")
            ]))?
            .build();

        // Add optional test mod
        if self.options.test {
            processor.add_custom_rules(vec![("mod_test","","")])
        }

        processor.from_file(Path::new(&self.options.input))?;

        if self.options.test {
             processor.print_result()?;
        }

        Ok(())
    }

    fn render(&self) -> Result<Option<PathBuf>, GdeError> {
        let out_file = match self.render_type {
            RenderType::Marp =>{
                marp::render( &self.options.format, &self.options.out_file)?
            }
            RenderType::MediaWiki => {
                mediawiki::render(self.options.test)?
            }
            RenderType::Pandoc => {
                pandoc::render(&self.options.out_file)?
            }
            RenderType::Gdlogue => {
                gdlogue::render(&self.options.format, &self.options.out_file)?
            }
            RenderType::FlowchartJs => {
                flowchartjs::render(&self.options.out_file)?
            }
            RenderType::FlowchartGvz => {
                flowchartgvz::render(&self.options.format,&self.options.out_file)?
            }
            RenderType::Webuibts => {
                webuibts::render(&self.options.out_file)?
            }
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
                    utils::CACHE_PATH.join("test_out.gddt")
                )?;
            } 

            // Copy option
            if let Some(path) = &self.options.copy {
                if path.is_dir() {
                    std::fs::rename(&final_file, path.join(&final_file.file_name().unwrap()))?;
                } else {
                    std::fs::rename(final_file, path)?;
                }
            }
        }
        // Renderer specific files
        match self.render_type {
            RenderType::MediaWiki => {
                let image_list = std::env::current_dir()?.join(mediawiki::IMAGE_LIST);
                // Test reseve image list file
                if image_list.exists() {
                    if self.options.test {
                        std::fs::rename(
                            image_list,
                            &*utils::CACHE_PATH.join(mediawiki::IMAGE_LIST)
                        )?;
                    } else {
                        std::fs::remove_file(image_list)?;
                    }
                }
            }
            _ => ()
        }

        // Cache preservation
        if !self.options.preserve {
            std::fs::remove_dir_all(utils::CACHE_PATH.to_owned())?;
            std::fs::create_dir(utils::CACHE_PATH.to_owned())?;
        }
        Ok(())
    }
}

pub struct ExecOptions {
    // Option used by rad
    strict: bool,
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
        lenient:bool,
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
            lenient,
            test,
            copy,
            format,
            input : input.to_str().unwrap().to_owned(),
            out_file: output,
        })
    }
}

pub enum RenderType {
    Marp,
    MediaWiki,
    Pandoc,
    Gdlogue,
    FlowchartJs,
    FlowchartGvz,
    Webuibts
}

impl RenderType {
    pub fn from(render_type : &str) -> Result<Self, GdeError> {
        let render_type = match render_type.to_lowercase().as_str() {
            "marp" => Self::Marp,
            "mediawiki" => Self::MediaWiki,
            "pandoc" => Self::Pandoc,
            "gdlogue" => Self::Gdlogue,
            "flowchartjs" => Self::FlowchartJs,
            "flowchartgvz" => Self::FlowchartGvz,
            "webuibts" => Self::Webuibts,
            _ => return Err(GdeError::InvalidCommand(format!("{} is not valid render type",render_type))),
        };
        Ok(render_type)
    }
}

use std::fmt;

impl fmt::Display for RenderType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &RenderType::Marp => "marp",
            &RenderType::MediaWiki => "mediawiki",
            &RenderType::Gdlogue => "gdlogue",
            &RenderType::Webuibts => "webuibts",
            &RenderType::Pandoc => "pandoc",
            &RenderType::FlowchartGvz => "flowchartgvz",
            &RenderType::FlowchartJs => "flowchartjs",
        };
        fmt.write_str(string)?;
        Ok(())
    }
}

