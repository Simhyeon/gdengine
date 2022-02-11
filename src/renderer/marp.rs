use std::path::{PathBuf, Path};
use crate::executor::ExecOptions;
use crate::utils;
use crate::models::GdeResult;
use std::ffi::OsStr;
use rad::{Processor, RadResult, MacroType};
use comrak::{markdown_to_html, ComrakOptions};
use crate::renderer::models::GRender;

pub struct MarpRenderer;

impl GRender for MarpRenderer {
    fn rad_setup(&self, processor : &mut Processor) -> GdeResult<()> {
        processor.add_basic_rules(vec![("mdtohtml",md_to_html as MacroType)]);

        Ok(())
    }
    fn render(&self, _: &mut Processor, option: &ExecOptions) -> GdeResult<Option<PathBuf>> {
        // Marp expects markdown file so, source file should be markdown
        let source_file = Path::new("out.md");
        // Copy file to root directory
        std::fs::copy(utils::middle_file_path()?, source_file)?;

        let format = if let Some(format) = &option.format {
            format.to_owned()
        } else {
            // Default is html
            "html".to_owned()
        };

        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join(format!("out.{}", format)).to_owned()
        };

        // If chrome path is not defined
        // Try setting new one
        if let Err(_) = std::env::var("CHROME_PATH") {
            let chrome_path = 
                // If chromium exists in env use it first
                if let Ok(path) = utils::renderer_bin_path("marp", "chromium") {
                    path
                } else {
                    // Else find chrome in path 
                    // and finally find chrome(chromium)
                    utils::renderer_bin_path("marp", Path::new("chrome").join("chrome"))?
                };

            // Update var only if it was not defined before
            std::env::set_var("CHROME_PATH", chrome_path);
        } 

        let marp_path: PathBuf = utils::renderer_bin_path("marp", "marp")?;

        utils::command(marp_path.to_str().unwrap(), vec![
            source_file.as_os_str(),
            OsStr::new("--allow-local-files"),
            OsStr::new("--html"),
            OsStr::new(&format!("--{}", format)),
            OsStr::new("-o"),
            out_file.as_os_str()
        ])?;

        // Source file is not necessary
        std::fs::remove_file(source_file)?;

        Ok(Some(out_file))
    }
}

/// Additional basic macro for md conversion
// Always greedy for consistency No need to utilzile processor
fn md_to_html(args: &str, _: bool, _ : &mut Processor) -> RadResult<Option<String>> {
    let mut comrak_option = ComrakOptions::default();
    // Enable raw html rendering
    comrak_option.render.unsafe_ = true;
    let converted = markdown_to_html(args, &comrak_option);
    Ok(Some(converted))
}
