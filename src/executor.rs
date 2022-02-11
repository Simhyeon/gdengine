use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::models::GdeResult;
use crate::utils;
use crate::renderer::*;
use rad::CommentType;
use rad::{Processor, AuthType, DiffOption, RadError};
use crate::renderer::models::GRender;

pub struct Executor {
    render_type: String,
    renderer: Box<dyn GRender>,
    options : ExecOptions,
    variable_list: Option<Vec<(String,String)>>
}

impl Executor {
    pub fn new(render_type: &str, options: ExecOptions, variable_list: Option<Vec<(String,String)>>) -> GdeResult<Self> {
        Ok(Self { 
            render_type: render_type.to_string(),
            renderer: Self::get_renderer(render_type,&options)?,
            options,
            variable_list,
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
        Ok(())
    }

    fn preprocess(&self, processor : &mut Processor) -> GdeResult<()> {
        self.renderer.rad_setup(processor)?;
        Ok(())
    }

    /// Build rad processor with options
    fn build_processor(&self) -> GdeResult<Processor> {
        let diff_option = if self.options.diff { DiffOption::Change } else { DiffOption::None };
        let mut processor = Processor::new()
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

        // Add variables
        processor.add_static_rules(
            self.variable_list.clone().unwrap_or(vec![])
        )?;

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
        let out_file = self.renderer.render(processor, &self.options)?;
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

    fn get_renderer(render_type: &str, option: &ExecOptions) -> GdeResult<Box<dyn GRender>> {
        Ok(match render_type {
            "marp" => Box::new(marp::MarpRenderer),
            "mediawiki" => {
                if option.test { Box::new(mediawiki::PreviewRenderer)} 
                else { Box::new(mediawiki::MWRenderer) }
            },
            "gdlouge" => Box::new(gdlogue::GDLogueRenderer),
            "pandoc" => Box::new(pandoc::PandocRenderer),
            "webuibts" => Box::new(webuibts::WBTSRenderer),
            "flowchartjs" => Box::new(flowchartjs::FJSRenderer),
            "flowchartgvz" => Box::new(flowchartgvz::FGVZRenderer),
            "plotters" => Box::new(plot::PlotRenderer),
            _ => return Err(GdeError::InvalidCommand(format!("Renderer \"{}\" is not a viable renderer", render_type))),
        })
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
    pub(crate) preserve: bool,
    pub(crate) format: Option<String>,
    pub(crate) out_file: Option<PathBuf>,
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
