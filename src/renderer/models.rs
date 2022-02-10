use std::path::PathBuf;

use rad::Processor;
use crate::models::GdeResult;

// Game design document renderer trait
pub(crate) trait GRender {
    fn render(&self, processor: &Processor, out_file: Option<PathBuf>, optional: Option<String>) -> GdeResult<Option<PathBuf>>;
    fn rad_setup(&self, processor : &mut Processor) -> GdeResult<()>;
}
