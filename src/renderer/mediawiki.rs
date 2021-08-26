use std::path::{PathBuf, Path};
use std::process::Command;
use crate::error::GdeError;
use crate::utils;

pub(crate) fn mediawiki_render(file: PathBuf, out_name: &Path) -> Result<(), GdeError> {
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
