use std::path::PathBuf;
use crate::error::GdeError;
use std::process::Command;
use crate::utils;
use rad::processor::Processor;
use rad::error::RadError;

pub(crate) fn render(format: &Option<String>, out_file: &Option<PathBuf>) -> Result<Option<PathBuf>, GdeError> {

    // Source file
    let source_file = utils::CACHE_PATH.join("out.json");

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
        utils::BUILD_PATH.join(format!("out.{}", format)).to_owned()
    };

    // Execute
    // This validates json file and yield out.gv in current working directory
    let mut output = Command::new("node")
        // Other aguments
        .arg(utils::renderer_path("gdlogue")?.join("index.js"))
        .arg(source_file)
        .arg("dotify")
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    match format.as_str() {
        "html" => {
            // This evaluates indes.html template
            // And create app.js with necessary data
            if let Err(err) = rad(&out_file) {
                eprintln!("{}", err);
            }
        }
        "pdf" => {
            output = Command::new("dot")
                .arg("-Tpdf")
                .arg("out.gv")
                .arg("-o")
                .arg(&out_file)
                .output()?;
        }
        "png" => {
            output = Command::new("dot")
                .arg("-Gdpi=300")
                .arg("-Tpng")
                .arg("out.gv")
                .arg("-o")
                .arg(&out_file)
                .output()?;
        }
        _ => {
            eprintln!("No usable format was given");
            return Ok(None);
        }
    }
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    std::fs::rename("out.gv", utils::CACHE_PATH.join("out.gv"))?;
    Ok(Some(out_file))
}

fn rad(out_file : &PathBuf) -> Result<(), RadError> {
    Processor::new()
        .greedy(true)
        .purge(true)
        .write_to_file(Some(out_file.to_owned()))?
        .from_file(&utils::renderer_path("gdlogue").expect("Failed to get renderer path").join("index.html"))?;

    Ok(())
}
