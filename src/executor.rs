use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::models::GdeResult;
use crate::utils;
use crate::renderer::*;
use rad::CommentType;
use rad::RadResult;
use rad::{Processor, AuthType, DiffOption, WriteOption};
use crate::renderer::models::GRender;

pub struct Executor {
    render_type: String,
    renderer: Box<dyn GRender>,
    options : ExecOption,
    variable_list: Option<Vec<(String,String)>>
}

impl Executor {
    pub fn new(render_type: &str, options: ExecOption, variable_list: Option<Vec<(String,String)>>) -> GdeResult<Self> {
        Ok(Self { 
            render_type: render_type.to_string(),
            renderer: Self::get_renderer(render_type)?,
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

        // Set toc automatically for mediawiki preview
        if self.render_type.as_str() == "mediawiki_preview"{
            self.options.toc = true; 
        }

        Ok(())
    }

    fn preprocess(&self, processor : &mut Processor) -> GdeResult<()> {
        self.renderer.rad_setup(processor)?;

        // Use toc macro variant
        // Only add h1 and h2
        if self.options.toc {
            processor.from_string(
                r#"$declare(toc_h1,toc_h2)
$append(h1,\* $append(TOC_LIST,1,$a_content()$nl()) *\ )
$append(h2,\* $append(TOC_LIST,2,$a_content()$nl()) *\ )"#)?;
        }

        Ok(())
    }

    /// Build rad processor with options
    fn build_processor(&self) -> GdeResult<Processor> {
        let diff_option = if self.options.diff { DiffOption::Change } else { DiffOption::None };
        let mut processor = Processor::new()
            .set_comment_type(CommentType::Start)
            // TODO How to handle lenient mode?
            .lenient(!self.options.strict)
            .purge(true)
            .log(self.options.log)
            .unix_new_line(true)
            .allow(Some(vec!(AuthType::ENV, AuthType::FIN, AuthType::FOUT, AuthType::CMD)))
            .write_to_file(Some(utils::middle_file_path().expect("Failed to get path")))?
            .melt_files(vec![
                    utils::STD_MACRO_PATH.to_owned(),
                    utils::module_path(self.render_type.to_string()).expect("Failed to get path")
            ])?
            .diff(diff_option)?;

        // Add variables
        processor.add_static_rules(
            self.variable_list.clone().unwrap_or(vec![])
        )?;

        // Add extension macros
        self.add_extension(&mut processor)?;

        Ok(processor)
    }

    // Add default extension macros
    fn add_extension(&self,processor : &mut Processor) -> RadResult<()> {
        crate::macro_extension::Extension::extend_all(processor);
        Ok(())
    }

    /// Expand macros from target source file
    fn macro_expansion(&self,processor : &mut Processor) -> RadResult<()> {
        // Add optional test mod
        if self.options.test {
            processor.add_static_rules(vec![("mod_test","")])?
        }

        // Process user custom file if exists
        // though this is not mandatory
        if utils::INDEX_RAD.exists() {
            processor.from_file(&*utils::INDEX_RAD)?;
        } 

        processor.from_file(Path::new(&self.options.input))?;

        if self.options.test | self.options.diff {
             processor.print_result()?;
        }

        // Process one more time if toc option was given
        if self.options.toc {
            let previous = utils::middle_file_path().expect("Failed to get path");
            let after = utils::middle_file_path_with_toc().expect("Failed to get path");
            std::fs::copy(&previous, &after)?;

            // Drop file handle from processor
            // This might not do anything, but it is letting the operating system or compiler to do
            // a graceful end of file handler opertion.
            processor.set_write_option(WriteOption::Discard);

            // Regain a file handle from middle file
            let file = std::fs::OpenOptions::new().truncate(true).write(true).open(previous)?;
            processor.set_write_option(WriteOption::File(file));
            processor.from_file(&after)?;
            // Because toc sets 
            processor.reset_flow_control();
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

    fn get_renderer(render_type: &str) -> GdeResult<Box<dyn GRender>> {
        Ok(match render_type {
            "marp" => Box::new(marp::MarpRenderer),
            "mediawiki" => Box::new(mediawiki::MWRenderer),
            "mediawiki_preview" => Box::new(mediawiki::PreviewRenderer),
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

// Executor Option
#[derive(Default)]
pub struct ExecOption {
    // Option used by rad
    input: String,
    strict: bool,
    test: bool,
    diff: bool,
    log: bool,
    toc: bool,
    // Used by post process
    copy: Option<PathBuf>,
    // Used by renderer
    pub(crate) preserve: bool,
    pub(crate) format: Option<String>,
    pub(crate) out_file: Option<PathBuf>,
}

impl ExecOption {
    pub fn new(input: Option<PathBuf>) -> Self {
        let mut obj = Self::default();
        let input = if let Some(content) = input {
            content
        } else {
            utils::DEFAULT_ENTRY_PATH.to_owned()
        };
        obj.input = input.to_str().unwrap().to_owned();

        obj
    }

    pub fn strict(mut self, tv: bool) -> Self {
        self.strict = tv;
        self
    }
    pub fn test(mut self, tv: bool) -> Self {
        self.test = tv;
        self
    }
    pub fn diff(mut self, tv: bool) -> Self {
        self.diff = tv;
        self
    }
    pub fn log(mut self, tv: bool) -> Self {
        self.log = tv;
        self
    }
    pub fn toc(mut self, tv: bool) -> Self {
        self.toc = tv;
        self
    }
    pub fn copy(mut self, path: Option<PathBuf>) -> Self {
        self.copy = path;
        self
    }
    pub fn preserve(mut self, tv: bool) -> Self {
        self.preserve = tv;
        self
    }
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    pub fn out_file(mut self, path: Option<PathBuf>) -> Self {
        self.out_file = path;
        self
    }
}
