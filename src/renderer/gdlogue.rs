use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::utils;
use rad::{Processor, RadError, AuthType};
use gdlogue::*;

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

    match format.as_str() {
        "html" => {
            // This evaluates index.html template
            // And creates app.js with necessary data
            if let Err(err) = html_dialogue(&out_file) {
                eprintln!("{}", err);
            }
        }
        "pdf" | "png" => {
            if let Err(err) = dot_image(&source_file, &format) {
                eprintln!("{}", err);
                return Err(GdeError::RendererError("Failed to create new dot image"));
            }

            // Move file contents into designated out file
            if format.as_str() == "pdf" {
                std::fs::rename("out.pdf", utils::BUILD_PATH.join(&out_file))?;
            } else {
                std::fs::rename("out.png", utils::BUILD_PATH.join(&out_file))?;
            }
        }
        _ => { eprintln!("No usable format was given"); }
    }
    // Move cached file
    std::fs::rename("out.gv", utils::CACHE_PATH.join("out.gv"))?;
    Ok(Some(out_file))
}

fn html_dialogue(out_file : &PathBuf) -> Result<(), RadError> {
    if let Err(err) = dot_file(&utils::CACHE_PATH.join("out.json")) {
        eprintln!("Err : {}", err);
    }

    Processor::new()
        .greedy(true)
        .write_to_file(Some(out_file.to_owned()))?
        .allow(Some(vec!(AuthType::FIN, AuthType::ENV)))
        .from_file(&utils::renderer_path("gdlogue").expect("Failed to get renderer path").join("index.html"))?;

    Ok(())
}

fn dot_file(path: &Path) -> Result<(), GdlError> {
    let file_content = Dialogue::read_from_file(path)?.dot_script("dialogue")?;
    std::fs::write(std::env::current_dir()?.join("out.gv"), file_content.as_bytes())?;
    Ok(())
}

fn dot_image(path: &Path, format: &str) -> Result<(), GdlError> {
    let format_enum: Format;
    match format {
        "png" => format_enum = Format::Png,
        "pdf" => format_enum = Format::Pdf,
        _ => {eprintln!("No proepr format was given"); return Ok(());}
    }
    Dialogue::new_dot_image(path, format_enum)?;

    Ok(())
}
