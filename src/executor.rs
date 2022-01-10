use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::models::GdeResult;
use crate::renderer::plot::PlotModel;
use crate::utils;
use crate::renderer::*;
use crate::renderer::mediawiki::ImageList;
use rad::CommentType;
use rad::StorageOutput;
use rad::{Processor, AuthType, DiffOption, RadError};

pub struct Executor {
    render_type: RenderType,
    options : ExecOptions,
}

impl Executor {
    pub fn new(render_type: &str, options: ExecOptions) -> GdeResult<Self> {
        Ok(Self { 
            render_type : RenderType::from(render_type)?,
            options,
        })
    }

    /// Main execution logic
    pub fn exec(&mut self) -> GdeResult<()> {
        // Make files if not present
        self.path_fallback()?;
        self.setup()?;

        let mut processor = self.build_processor()?;
        self.preprocess(&mut processor)?;

        // Macro expansion
        if let Err(err) = self.macro_expansion(&mut processor) {
            eprintln!("{}", err);
            if self.options.test {
                eprintln!("{}", err);
                return Ok(());
            } else {
                return Err(GdeError::Raderror(err));
            }
        }

        // Post process
        let out_file = match self.render(&mut processor) {
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

    // Crate file and directories if not existent
    fn path_fallback(&self) -> GdeResult<()> {
        // Crate build directory
        if !utils::BUILD_PATH.exists() {
            std::fs::create_dir(&*utils::BUILD_PATH)?;
        }
        // Crate cache directory
        if !utils::CACHE_PATH.exists() {
            std::fs::create_dir(&*utils::CACHE_PATH)?;
        }
        Ok(())
    }

    /// Setup necessary information
    fn setup(&mut self) -> GdeResult<()> {
        std::env::set_var("GDE_MODULE", utils::renderer_path(self.render_type.to_string())?);

        // Render type specific pre-processing logics
        match self.render_type {
            RenderType::MediaWiki => {
                if self.options.test {
                    self.render_type = RenderType::MWPreview;
                }
            }
            _ =>()
        }

        Ok(())
    }

    fn preprocess(&self, processor : &mut Processor) -> GdeResult<()> {
        // Render type specific pre-processing logics
        match self.render_type {
            RenderType::MediaWiki => {
                mediawiki::rad_setup(processor)?;
            }
            RenderType::Marp => {
                marp::rad_setup(processor)?;
            }
            RenderType::Plotter => {
                plot::rad_setup(processor)?;
            }
            _ =>()
        }

        Ok(())
    }

    /// Build rad processor with options
    fn build_processor(&self) -> GdeResult<Processor> {
        let diff_option = if self.options.diff { DiffOption::Change } else { DiffOption::None };
        let processor = Processor::new()
            .purge(true)
            .greedy(true)
            .set_comment_type(CommentType::Start)
            .lenient(!self.options.strict)
            .log(self.options.log)
            .unix_new_line(true)
            .allow(Some(vec!(AuthType::ENV, AuthType::FIN, AuthType::FOUT, AuthType::CMD)))
            .write_to_file(Some(utils::middle_file_path().expect("Failed to get path")))?
            .rule_files(Some(vec![
                    utils::STD_MACRO_PATH.to_owned(),
                    utils::module_path(self.render_type.to_string()).expect("Failed to get path")
            ]))?
            .diff(diff_option)?;

        Ok(processor)
    }

    /// Expand macros from target source file
    fn macro_expansion(&self,processor : &mut Processor) -> Result<(), RadError> {
        // Add optional test mod
        if self.options.test {
            processor.add_static_rules(vec![("mod_test","")])?
        }

        if utils::INDEX_RAD.exists() {
            // TODO
            processor.from_file(&*utils::INDEX_RAD)?;
        }

        processor.from_file(Path::new(&self.options.input))?;

        if self.options.test | self.options.diff {
             processor.print_result()?;
        }

        Ok(())
    }

    /// Render with a designated renderer
    fn render(&self, processor: &mut Processor) -> GdeResult<Option<PathBuf>> {
        let out_file = match self.render_type {
            RenderType::Marp =>{
                marp::render( &self.options.format, &self.options.out_file)?
            }
            RenderType::MediaWiki => {
                let output = processor.extract_storage(true).unwrap().unwrap();
                let image_list: ImageList;
                if let StorageOutput::Binary(bytes) = output {
                    image_list = ImageList::from_bytes(bytes)?;
                } else {
                    return Err(GdeError::RendererError("Image list cannot be constructed from StorageOutput::Text"));
                }
                mediawiki::render(image_list)?
            }
            RenderType::MWPreview => {
                mediawiki::render_preview()?
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
            RenderType::Plotter => {
                // Extract storage method should always return Some
                // unless, it is a logic error
                let plot_model = if let Ok(Some(output)) = processor.extract_storage(true) {
                    if let StorageOutput::Binary(bytes) = output {
                        PlotModel::from_bytes(&bytes)?
                    } else {
                        return Err(GdeError::RendererError("Plotmodel cannot be constructed from StorageOutput::Text"));
                    }
                } else { 
                    return Err(GdeError::RendererError("Plot needs porper macro setup to work. Failed to created plot map"));
                };
                plot::render(&self.options.out_file, plot_model).expect("DEBUG")
            }
        };
        
        Ok(out_file)
    }

    // Copy output file to a designated path
    // Remove cached file
    fn postprocess(&self, final_file: Option<PathBuf>) -> GdeResult<()> {
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
    diff: bool,
    log: bool,
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
        strict:bool,
        test:bool,
        diff:bool,
        log:bool,
        copy: Option<PathBuf>,
        format: Option<String>,
        input: Option<PathBuf>,
        output: Option<PathBuf>
    ) -> GdeResult<Self> {
        let input = if let Some(content) = input {
            content
        } else {
            utils::DEFAULT_ENTRY_PATH.to_owned()
        };

        Ok(Self { 
            preserve,
            strict,
            test,
            diff,
            log,
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
    MWPreview,
    Pandoc,
    Gdlogue,
    FlowchartJs,
    FlowchartGvz,
    Webuibts,
    Plotter,
}

impl RenderType {
    pub fn from(render_type : &str) -> GdeResult<Self> {
        let render_type = match render_type.to_lowercase().as_str() {
            "marp" => Self::Marp,
            "mediawiki" => Self::MediaWiki,
            "pandoc" => Self::Pandoc,
            "gdlogue" => Self::Gdlogue,
            "flowchartjs" => Self::FlowchartJs,
            "flowchartgvz" => Self::FlowchartGvz,
            "webuibts" => Self::Webuibts,
            "plotter" => Self::Plotter,
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
            &RenderType::MWPreview => "mediawiki_preview",
            &RenderType::Gdlogue => "gdlogue",
            &RenderType::Webuibts => "webuibts",
            &RenderType::Pandoc => "pandoc",
            &RenderType::FlowchartGvz => "flowchartgvz",
            &RenderType::FlowchartJs => "flowchartjs",
            &RenderType::Plotter => "plotter",
        };
        fmt.write_str(string)?;
        Ok(())
    }
}

