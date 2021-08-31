use std::path::PathBuf;
use std::process::Command;
use crate::utils;
use crate::error::GdeError;

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

    let output = Command::new("marp")
        .arg(&source_file)
        .arg("--allow-local-files")
        .arg("--html")
        .arg(format!("--{}",format))
        .arg("-o")
        .arg(&out_file)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(Some(out_file))
}
