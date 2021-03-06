use super::models::GRender;
use crate::error::GdeError;
use crate::executor::ExecOption;
use crate::models::GdeResult;
use crate::utils;
use gdlogue::*;
use r4d::{Processor, RadResult, WriteOption};
use std::path::{Path, PathBuf};

pub struct GDLogueRenderer;

impl GRender for GDLogueRenderer {
    fn rad_setup(&self, _: &mut Processor) -> GdeResult<()> {
        Ok(())
    }

    fn render(&self, p: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>> {
        // Source file
        let source_file = utils::CACHE_PATH.join("out.json");

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
            utils::BUILD_PATH.join(format!("out.{}", format))
        };

        match format.as_str() {
            "html" => {
                // This evaluates index.html template
                // And creates app.js with necessary data
                if let Err(err) = self.html_dialogue(p, &out_file) {
                    eprintln!("{}", err);
                }
            }
            "pdf" | "png" => {
                if let Err(err) = self.dot_image(&source_file, &format) {
                    eprintln!("{}", err);
                    return Err(GdeError::RendererError("Failed to create new dot image"));
                }

                // Move file contents into designated out file
                if format.as_str() == "pdf" {
                    std::fs::rename("out.pdf", utils::BUILD_PATH.join(&out_file))?;
                } else {
                    std::fs::rename("out.png", utils::BUILD_PATH.join(&out_file))?;
                }
            }
            _ => {
                eprintln!("No usable format was given");
            }
        }
        // Move cached file
        std::fs::rename("out.gv", utils::CACHE_PATH.join("out.gv"))?;
        Ok(Some(out_file))
    }
}

impl GDLogueRenderer {
    fn html_dialogue(&self, p: &mut Processor, out_file: &PathBuf) -> RadResult<()> {
        if let Err(err) = self.dot_file(&utils::CACHE_PATH.join("out.json")) {
            eprintln!("Err : {}", err);
        }

        let new_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(out_file)?;

        p.set_write_option(WriteOption::File(new_file));
        p.process_file(
            &utils::renderer_path("gdlogue")
                .expect("Failed to get renderer path")
                .join("index.html"),
        )?;

        Ok(())
    }

    fn dot_file(&self, path: &Path) -> Result<(), GdlError> {
        let file_content = Dialogue::read_from_file(path)?.dot_script("dialogue")?;
        std::fs::write(
            std::env::current_dir()?.join("out.gv"),
            file_content.as_bytes(),
        )?;
        Ok(())
    }

    fn dot_image(&self, path: &Path, format: &str) -> Result<(), GdlError> {
        let format_enum = match format {
            "png" => Format::Png,
            "pdf" => Format::Pdf,
            _ => {
                eprintln!("No proepr format was given");
                return Ok(());
            }
        };
        Dialogue::new_dot_image(path, format_enum)?;

        Ok(())
    }
}
