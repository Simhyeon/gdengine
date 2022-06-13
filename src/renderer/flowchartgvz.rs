use crate::error::GdeError;
use r4d::{Processor, RadStorage, StorageOutput};
use std::path::PathBuf;

use crate::executor::ExecOption;
use crate::models::GdeResult;
use crate::utils;
use std::ffi::OsStr;

use super::models::GRender;

pub struct FGVZRenderer; // Not yet

impl GRender for FGVZRenderer {
    fn rad_setup(&self, processor: &mut Processor) -> GdeResult<()> {
        processor.set_storage(Box::new(FlowchartSrc {
            dot_src: String::new(),
        }));
        Ok(())
    }

    fn render(&self, processor: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>> {
        let output = processor.extract_storage(false).unwrap().unwrap();
        let dot_src = if let StorageOutput::Text(texts) = output {
            texts
        } else {
            return Err(GdeError::RendererError(
                "Dot source cannot be constructed from StorageOutput::Text",
            ));
        };

        // Set default format, which is png
        let format = if let Some(format) = &option.format {
            format.to_owned()
        } else {
            "png".to_owned()
        };

        // Set default outfile
        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join(&format!("out.{}", format))
        };

        match format.as_str() {
            "pdf" => {
                utils::command_with_stdin(
                    "dot",
                    vec![OsStr::new("-Tpdf"), OsStr::new("-o"), out_file.as_os_str()],
                    &dot_src,
                )?;
            }
            "png" => {
                utils::command_with_stdin(
                    "dot",
                    vec![
                        OsStr::new("-Gdpi=300"),
                        OsStr::new("-Tpng"),
                        OsStr::new("-o"),
                        out_file.as_os_str(),
                    ],
                    &dot_src,
                )?;
            }
            _ => {
                eprintln!("No usable format was given");
            }
        }

        Ok(Some(out_file))
    }
}

pub struct FlowchartSrc {
    pub dot_src: String,
}

impl RadStorage for FlowchartSrc {
    fn update(&mut self, args: &[String]) -> r4d::StorageResult<()> {
        self.dot_src.push_str(&args[0]);
        Ok(())
    }

    fn extract(&mut self, _: bool) -> r4d::StorageResult<Option<r4d::StorageOutput>> {
        Ok(Some(r4d::StorageOutput::Text(self.dot_src.clone())))
    }
}
