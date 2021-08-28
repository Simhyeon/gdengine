use std::path::{PathBuf, Path};
use crate::error::GdeError;
use std::process::Command;
use crate::utils;

pub(crate) fn render(format: &Option<String>, out_file: &Option<PathBuf>) -> Result<(), GdeError> {

    // Sourc file, literaly
    let source_file = utils::middle_file_path()?;

    // Set default format
    let format = if let Some(format) = format {
        format.to_owned()
    } else {
        "png".to_owned()
    };

    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join(format!("out.{}", format)).to_owned()
    };

    // Execute
    let mut output = Command::new("node")
        // Other aguments
        .arg(utils::renderer_path("gdlogue")?.join("index.js"))
        .arg(source_file)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    match format.as_str() {
        "html" => {
            output = Command::new("rad")
                .arg(utils::renderer_path("gdlogue")?.join("index.html"))
                .arg(out_file)
                .output()?;
            }
        "pdf" => {
            output = Command::new("dot")
                .arg("-Tpdf")
                .arg("out.gv")
                .arg(out_file)
                .output()?;
            }
        "png" => {
            output = Command::new("dot")
                .arg("-Tpng")
                .arg("out.gv")
                .arg(out_file)
                .output()?;
            }
    }
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}
