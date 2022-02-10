use std::path::PathBuf;
use rad::{RadStorage,Processor};

use crate::utils;
use crate::models::GdeResult;
use std::ffi::OsStr;

//pub struct FGVZRenderer; // Not yet
pub fn rad_setup(processor : &mut Processor) -> GdeResult<()> {
    processor.set_storage(Box::new(FlowchartSrc { dot_src: String::new() }));
    Ok(())
}

pub fn render(format: &Option<String>, out_file: &Option<PathBuf>, dot_src: &str) -> GdeResult<Option<PathBuf>> {
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

    match format.as_str() {
        "pdf" => {
            utils::command_with_stdin("dot", vec![
                OsStr::new("-Tpdf"),
                OsStr::new("-o"),
                out_file.as_os_str()
            ], dot_src)?;
        }
        "png" => {
            utils::command_with_stdin("dot", vec![
                OsStr::new("-Gdpi=300"),
                OsStr::new("-Tpng"),
                OsStr::new("-o"),
                out_file.as_os_str()
            ], dot_src)?;
        }
        _ => { eprintln!("No usable format was given"); }
    }

    Ok(Some(out_file))
}

pub struct FlowchartSrc {
    pub dot_src: String,
}

impl RadStorage for FlowchartSrc {
    fn update(&mut self, args : &Vec<String>) -> rad::StorageResult<()> {
        self.dot_src.push_str(&args[0]);
        Ok(())
    }

    fn extract(&mut self, _: bool) -> rad::StorageResult<Option<rad::StorageOutput>> {
        Ok(Some(rad::StorageOutput::Text(self.dot_src.clone())))
    }
}

