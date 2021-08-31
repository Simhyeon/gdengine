use std::path::PathBuf;
use std::process::Command;
use crate::utils;
use crate::error::GdeError;

pub(crate) fn render(out_file: &Option<PathBuf>) -> Result<Option<PathBuf>, GdeError> {
    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join("out.html").to_owned()
    };

    let output = Command::new("rad")
        .arg(utils::renderer_path("flowchartjs")?.join("index.html"))
        .arg("-o")
        .arg(&out_file)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(Some(out_file))
}
