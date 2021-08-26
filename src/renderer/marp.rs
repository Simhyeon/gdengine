use std::path::{PathBuf, Path};
use std::process::Command;
use crate::error::GdeError;

pub(crate) fn marp_render(file: PathBuf, format: &str, out_name: &Path) -> Result<(), GdeError> {
    let output = Command::new("marp")
        .arg(file)
        .arg("--allow-local-files")
        .arg("--html")
        .arg(format!("--{}",format))
        .arg("-o")
        .arg(out_name)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
