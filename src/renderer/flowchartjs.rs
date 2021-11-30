use std::path::PathBuf;
use crate::utils;
use crate::models::GdeResult;
use rad::{Processor, RadResult, AuthType};

pub(crate) fn render(out_file: &Option<PathBuf>) -> GdeResult<Option<PathBuf>> {
    // Set default outfile
    let out_file = if let Some(name) = out_file {
        name.to_owned()
    } else {
        utils::BUILD_PATH.join("out.html").to_owned()
    };

    if let Err(err) = rad(&out_file) {
        eprintln!("{}", err);
    }

    Ok(Some(out_file))
}

fn rad(out_file : &PathBuf) -> RadResult<()> {
    Processor::new()
        .greedy(true)
        .allow(Some(vec!(AuthType::FIN, AuthType::ENV)))
        .unix_new_line(true)
        .write_to_file(Some(out_file.to_owned()))?
        .from_file(&utils::renderer_path("flowchartjs").expect("Failed to get path").join("index.html"))?;

    Ok(())
}
