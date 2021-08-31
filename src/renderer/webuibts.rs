use std::path::PathBuf;
use std::process::Command;
use crate::utils;
use crate::error::GdeError;

pub(crate) fn render(out_file: &Option<PathBuf>) -> Result<(), GdeError> {
    let source_file = utils::CACHE_PATH.join("flowchartgvz_source.gvz");

    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join("out.html").to_owned()
    };

    // TODO
    let output = Command::new("rad")
        .arg(utils::renderer_path("webuibts")?.join("index.html"))
        .arg("-m")
        .arg(utils::module_path("webuibts")?)
        .arg("-o")
        .arg(out_file)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
