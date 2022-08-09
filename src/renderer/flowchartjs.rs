use super::models::GRender;
use crate::executor::ExecOption;
use crate::models::GdeResult;
use crate::utils;
use r4d::{Processor, RadResult, WriteOption};
use std::path::{Path, PathBuf};

pub struct FJSRenderer;

impl GRender for FJSRenderer {
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

impl FJSRenderer {
    fn rad(&self, p: &mut Processor, out_file: &Path) -> RadResult<()> {
        let target = WriteOption::file(
            out_file,
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .clone(),
        )?;
        p.set_write_option(target);
        p.process_file(
            &utils::renderer_path("flowchartjs")
                .expect("Failed to get path")
                .join("index.html"),
        )?;

        Ok(())
    }
}
