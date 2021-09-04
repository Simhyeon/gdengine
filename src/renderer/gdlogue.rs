use std::path::PathBuf;
use crate::error::GdeError;
use crate::utils;
use rad::processor::Processor;
use rad::error::RadError;
use std::ffi::OsStr;

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
    utils::command("node", vec![
        utils::renderer_path("gdlogue")?.join("index.js").as_os_str(),
        source_file.as_os_str(),
        OsStr::new("dotify")
    ])?;

    match format.as_str() {
        "html" => {
            // This evaluates index.html template
            // And creates app.js with necessary data
            if let Err(err) = rad(&out_file) {
                eprintln!("{}", err);
            }
        }
        "pdf" => {
            utils::command("dot", vec![
                OsStr::new("-Tpdf"),
                OsStr::new("out.gv"),
                OsStr::new("-o"),
                out_file.as_os_str()
            ])?;
        }
        "png" => {
            utils::command("dot", vec![
                OsStr::new("-Gdpi=300"),
                OsStr::new("-Tpng"),
                OsStr::new("out.gv"),
                OsStr::new("-o"),
                out_file.as_os_str()
            ])?;
        }
        _ => { eprintln!("No usable format was given"); }
    }
    std::fs::rename("out.gv", utils::CACHE_PATH.join("out.gv"))?;
    Ok(Some(out_file))
}

fn rad(out_file : &PathBuf) -> Result<(), RadError> {
    Processor::new()
        .greedy(true)
        .write_to_file(Some(out_file.to_owned()))?
        .from_file(&utils::renderer_path("gdlogue").expect("Failed to get renderer path").join("index.html"))?;

    Ok(())
}
