use std::path::{PathBuf, Path};
use crate::utils;
use crate::models::GdeResult;
use std::ffi::OsStr;
use rad::{Processor, RadResult, MacroType};
use comrak::{markdown_to_html, ComrakOptions};

pub(crate) fn rad_setup(processor : &mut Processor) -> GdeResult<()> {
    processor.add_basic_rules(vec![("mdtohtml",md_to_html as MacroType)]);

    Ok(())
}

// Always greedy for consistency No need to utilzile processor
fn md_to_html(args: &str, _: bool, _ : &mut Processor) -> RadResult<Option<String>> {
    let mut comrak_option = ComrakOptions::default();
    // Enable raw html rendering
    comrak_option.render.unsafe_ = true;
    let converted = markdown_to_html(args, &comrak_option);
    Ok(Some(converted))
}

pub(crate) fn render(format: &Option<String>, out_file: &Option<PathBuf>) -> GdeResult<Option<PathBuf>> {

    // Marp expects markdown file so, source file should be markdown
    let source_file = Path::new("out.md");
    // Copy file to root directory
    std::fs::copy(utils::middle_file_path()?, source_file)?;

    let format = if let Some(format) = format {
        format.to_owned()
    } else {
        "html".to_owned()
    };

    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join(format!("out.{}", format)).to_owned()
    };

    // Set local chrome path for marp
    let chrome_name: &str;
    if cfg!(debug_assertions) {
        chrome_name = "chrome";
    } else {
        if cfg!(target_os = "windows") {
            chrome_name= "chrome.exe";
        } else {
            chrome_name= "chrome";
        }
    }
    let chrome_path = utils::renderer_path("marp")?.join("bin").join("chrome").join(chrome_name);
    std::env::set_var("CHROME_PATH", chrome_path);

    let marp_path: PathBuf;
    if cfg!(debug_assertions) {
        marp_path = utils::renderer_path("marp")?.join("bin").join("marp");
    } else {
        if cfg!(target_os = "windows") {
            marp_path = utils::renderer_path("marp")?.join("bin").join("marp.exe");
        } else {
            marp_path = utils::renderer_path("marp")?.join("bin").join("marp");
        }
    }

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
