use std::path::{PathBuf, Path};
use std::process::Command;
use crate::error::GdeError;
use crate::utils;

pub(crate) fn marp_render(file: PathBuf, format: Option<String>, out_name: &Path) -> Result<(), GdeError> {
    // Set file format
    let format = if let None = format {
        "".to_owned()
    } else {
        format!("{}",format.unwrap())
    };

    let output = Command::new("marp")
        .arg(file)
        .arg("--allow-local-files")
        .arg("--html")
        .arg(format!("--{}",format))
        .arg("-o")
        .arg(format!("{}.{}",utils::cache_path()?.join("out").display(),format))
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
