use std::path::PathBuf;
use crate::utils;
use crate::models::GdeResult;
use crate::executor::ExecOption;
use rad::{Processor, RadResult, WriteOption};
use super::models::GRender;

pub struct FJSRenderer;

impl GRender for FJSRenderer {
    fn rad_setup(&self, _ : &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, p: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>> {
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

impl FJSRenderer {
    fn rad(&self,p: &mut Processor, out_file : &PathBuf) -> RadResult<()> {
        let new_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(out_file)?;

        p.set_write_option(WriteOption::File(new_file));
        p.from_file(&utils::renderer_path("flowchartjs").expect("Failed to get path").join("index.html"))?;

        Ok(())
    }
}

