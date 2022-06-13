use super::models::GRender;
use crate::executor::ExecOption;
use crate::models::GdeResult;
use crate::utils;
use r4d::Processor;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct PandocRenderer;

impl GRender for PandocRenderer {
    fn rad_setup(&self, _: &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, _: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>> {
        // Set default outfile
        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join("out.docx")
        };

        let pandoc_path: PathBuf = utils::renderer_bin_path("pandoc", "pandoc")?;

        utils::command(
            pandoc_path.to_str().unwrap(),
            vec![
                utils::middle_file_path()?.as_os_str(),
                OsStr::new("--from"),
                OsStr::new("markdown+raw_attribute"),
                OsStr::new("-o"),
                out_file.as_os_str(),
            ],
        )?;

        Ok(Some(out_file))
    }
}
