use std::path::PathBuf;
use crate::utils;
use crate::models::GdeResult;
use crate::executor::ExecOptions;
use rad::{Processor, RadResult, AuthType};
use super::models::GRender;

pub struct FJSRenderer;

impl GRender for FJSRenderer {
    fn rad_setup(&self, _ : &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, _: &mut Processor, option: &ExecOptions) -> GdeResult<Option<PathBuf>> {
        // Set default outfile
        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join("out.html").to_owned()
        };

        if let Err(err) = self.rad(&out_file) {
            eprintln!("{}", err);
        }

        Ok(Some(out_file))
    }
}

impl FJSRenderer {
    fn rad(&self, out_file : &PathBuf) -> RadResult<()> {
        Processor::new()
            .greedy(true)
            .allow(Some(vec!(AuthType::FIN, AuthType::ENV)))
            .unix_new_line(true)
            .write_to_file(Some(out_file.to_owned()))?
            .from_file(&utils::renderer_path("flowchartjs").expect("Failed to get path").join("index.html"))?;

        Ok(())
    }
}

