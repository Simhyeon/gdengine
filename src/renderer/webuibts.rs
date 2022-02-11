use std::path::PathBuf;
use crate::utils;
use crate::models::GdeResult;
use rad::{Processor, RadResult, WriteOption};
use crate::executor::ExecOptions;

use super::models::GRender;

pub struct WBTSRenderer;

impl GRender for WBTSRenderer {
    fn rad_setup(&self, _ : &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, p: &mut Processor, option: &ExecOptions) -> GdeResult<Option<PathBuf>> {
        // Set default outfile
        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join("out.html").to_owned()
        };

        if let Err(err) = self.rad(p,&out_file) {
            eprintln!("{}", err);
        }

        Ok(Some(out_file))
    }
}

impl WBTSRenderer {
    fn rad(&self,p: &mut Processor, out_file : &PathBuf) -> RadResult<()> {
        let new_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(out_file)?;

        p.set_write_option(WriteOption::File(new_file));
        p.from_file(&utils::renderer_path("webuibts").expect("Failed to get renderer path").join("index.html"))?;

        Ok(())
    }
}
