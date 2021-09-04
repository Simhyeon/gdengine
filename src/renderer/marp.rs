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

    utils::command("marp", vec![
        source_file.as_os_str(),
        OsStr::new("--allow-local-files"),
        OsStr::new("--html"),
        OsStr::new(&format!("--{}", format)),
        OsStr::new("-o"),
        out_file.as_os_str()
    ])?;

    Ok(Some(out_file))
}
