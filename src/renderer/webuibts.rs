use std::path::PathBuf;
use crate::utils;
use crate::models::GdeResult;
use rad::{AuthType, Processor, RadResult};
use crate::executor::ExecOptions;

use super::models::GRender;

pub struct WBTSRenderer;

impl GRender for WBTSRenderer {
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

impl WBTSRenderer {
    fn rad(&self, out_file : &PathBuf) -> RadResult<()> {
        Processor::new()
            .greedy(true)
            .write_to_file(Some(out_file.to_owned()))?
            .unix_new_line(true)
            .allow(Some(vec!(AuthType::FIN, AuthType::ENV)))
            .rule_files(Some(
                    vec![utils::module_path("webuibts").expect("Failed to get module path")]
            ))?
            .from_file(&utils::renderer_path("webuibts").expect("Failed to get renderer path").join("index.html"))?;

        Ok(())
    }
}
