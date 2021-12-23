use std::path::PathBuf;
use std::ffi::OsStr;
use crate::utils;
use crate::models::GdeResult;

pub(crate) fn render(out_file: &Option<PathBuf>) -> GdeResult<Option<PathBuf>> {
    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join("out.docx").to_owned()
    };

    let pandoc_path: PathBuf = utils::renderer_bin_path("pandoc", "pandoc")?;

    utils::command(pandoc_path.to_str().unwrap(), vec![
        utils::middle_file_path()?.as_os_str(),
        OsStr::new("--from"),
        OsStr::new("markdown+raw_attribute"),
        OsStr::new("-o"),
        out_file.as_os_str()
    ])?;

    Ok(Some(out_file))
}
