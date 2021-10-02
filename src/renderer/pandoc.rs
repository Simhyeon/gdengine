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

    let pandoc_path: PathBuf;
    if cfg!(debug_assertions) {
        pandoc_path = utils::renderer_path("pandoc")?.join("bin").join("pandoc");
    } else {
        if cfg!(target_os = "windows") {
            pandoc_path = std::env::current_exe()?.join("pandoc.exe");
        } else {
            pandoc_path = std::env::current_exe()?.join("pandoc");
        }
    }

    utils::command(pandoc_path.to_str().unwrap(), vec![
        utils::middle_file_path()?.as_os_str(),
        OsStr::new("--from"),
        OsStr::new("markdown+raw_attribute"),
        OsStr::new("-o"),
        out_file.as_os_str()
    ])?;

    Ok(Some(out_file))
}