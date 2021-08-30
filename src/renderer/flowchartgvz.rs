use std::path::PathBuf;
use std::process::Command;
use crate::utils;
use crate::error::GdeError;

pub(crate) fn render(format: &Option<String>,out_file: &Option<PathBuf>) -> Result<(), GdeError> {
    let source_file = utils::CACHE_PATH.join("flowchartgvz_source.gvz");

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

    let output = match format.as_str() {
        "pdf" => {
            Command::new("dot")
                .arg("-Tpdf")
                .arg(source_file)
                .arg("-o")
                .arg(out_file)
                .output()?
            }
        "png" => {
            Command::new("dot")
                .arg("-Gdpi=300")
                .arg("-Tpng")
                .arg(source_file)
                .arg("-o")
                .arg(out_file)
                .output()?
            }
        _ => {
            eprintln!("No usable format was given");
            return Ok(());
        }
    };

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
