use std::path::PathBuf;
use crate::utils;
use crate::error::GdeError;
use crate::models::GdeResult;
use std::ffi::OsStr;

pub(crate) fn render(format: &Option<String>,out_file: &Option<PathBuf>) -> GdeResult<Option<PathBuf>> {
    // NOTE
    // Source_file may fail because source file is created only if flowchart macro was invoked.
    let source_file = utils::CACHE_PATH.join("flowchartgvz_source.gvz");

    // No source file, return proper error
    if !source_file.exists() {
        return Err(GdeError::NoSuchPath(format!("Dot file : {} doesn't exit. Did you call flowchart macro?",source_file.display())));
    }

    // Set default format, which is png
    let format = if let Some(format) = format {
        format.to_owned()
    } else {
        "png".to_owned()
    };

    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join(&format!("out.{}",format)).to_owned()
    };

    match format.as_str() {
        "pdf" => {
            utils::command("dot", vec![
                OsStr::new("-Tpdf"),
                source_file.as_os_str(),
                OsStr::new("-o"),
                out_file.as_os_str()
            ])?;
        }
        "png" => {
            utils::command("dot", vec![
                OsStr::new("-Gdpi=300"),
                OsStr::new("-Tpng"),
                source_file.as_os_str(),
                OsStr::new("-o"),
                out_file.as_os_str()
            ])?;
        }
        _ => { eprintln!("No usable format was given"); }
    }

    Ok(Some(out_file))
}
