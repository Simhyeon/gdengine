use crate::executor::ExecOption;
use crate::models::GdeResult;
use crate::utils;
use r4d::{Processor, RadResult, WriteOption};
use std::path::PathBuf;

use super::models::GRender;

pub struct WBTSRenderer;

impl GRender for WBTSRenderer {
    fn rad_setup(&self, _: &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, p: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>> {
        // Set default outfile
        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join("out.html")
        };

        if let Err(err) = self.rad(p, &out_file) {
            eprintln!("{}", err);
        }

        Ok(Some(out_file))
    }
}

impl WBTSRenderer {
    fn rad(&self, p: &mut Processor, out_file: &PathBuf) -> RadResult<()> {
        let new_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(out_file)?;

        p.set_write_option(WriteOption::File(new_file));
        p.process_file(
            &utils::renderer_path("webuibts")
                .expect("Failed to get renderer path")
                .join("index.html"),
        )?;

        Ok(())
    }
}
