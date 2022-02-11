use std::path::PathBuf;
use std::ffi::OsStr;
use rad::Processor;
use crate::utils;
use crate::models::GdeResult;
use super::models::GRender;
use crate::executor::ExecOptions;

pub struct PandocRenderer;

impl GRender for PandocRenderer {
    fn rad_setup(&self, _ : &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, _: &mut Processor, option: &ExecOptions) -> GdeResult<Option<PathBuf>> {
        // Set default outfile
        let out_file = if let Some(name) = &option.out_file {
            name.to_owned()
        } else {
            utils::BUILD_PATH.join("out.docx").to_owned()
        };

        let pandoc_path: PathBuf = utils::renderer_bin_path("pandoc", "pandoc")?;

        utils::command(pandoc_path.to_str().unwrap(), vec![
            utils::middle_file_path()?.as_os_str(),
            OsStr::new("--from"),
            OsStr::new("markdown+raw_attribute"),
            OsStr::new("-o"),
            out_file.as_os_str()
        ])?;

        Ok(Some(out_file))
    }
}
