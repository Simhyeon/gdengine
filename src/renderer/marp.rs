use std::path::PathBuf;
use crate::utils;
use crate::error::GdeError;
use std::ffi::OsStr;

pub(crate) fn render(format: &Option<String>, out_file: &Option<PathBuf>) -> Result<Option<PathBuf>, GdeError> {
    // Change name to out.md
    let mut source_file = utils::middle_file_path()?;
    source_file.set_extension("md");
    std::fs::rename(utils::middle_file_path()?, &source_file)?;

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
    if cfg!(debug_assertions) {
        std::env::set_var("CHROME_PATH", utils::renderer_path("marp")?.join("bin").join("chrome").join("chrome"));
    } else {
        let chrome_name: &str;
        if cfg!(target_os = "windows") {
            chrome_name= "chrome.exe";
        } else {
            chrome_name= "chrome";
        }
        std::env::set_var("CHROME_PATH", std::env::current_exe()?.join(chrome_name));
    }

    let marp_path: PathBuf;
    if cfg!(debug_assertions) {
        marp_path = utils::renderer_path("marp")?.join("bin").join("marp");
    } else {
        if cfg!(target_os = "windows") {
            marp_path = std::env::current_exe()?.join("marp.exe");
        } else {
            marp_path = std::env::current_exe()?.join("marp");
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

    Ok(Some(out_file))
}
