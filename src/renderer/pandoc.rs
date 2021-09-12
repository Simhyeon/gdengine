use std::path::PathBuf;
use std::ffi::OsStr;
use crate::utils;
use crate::error::GdeError;

pub(crate) fn render(out_file: &Option<PathBuf>) -> Result<Option<PathBuf>, GdeError> {
    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join("out.docx").to_owned()
    };

    utils::command("pandoc", vec![
        utils::middle_file_path()?.as_os_str(),
        OsStr::new("--from"),
        OsStr::new("markdown+raw_attribute"),
        OsStr::new("-o"),
        out_file.as_os_str()
    ])?;

    Ok(Some(out_file))
}
