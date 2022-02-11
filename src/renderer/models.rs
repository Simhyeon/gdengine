use std::path::PathBuf;

use rad::Processor;
use crate::{models::GdeResult, executor::ExecOptions};

// Game design document renderer trait
pub(crate) trait GRender {
    fn render(&self, processor: &mut Processor, option: &ExecOptions) -> GdeResult<Option<PathBuf>>;
    fn rad_setup(&self, processor : &mut Processor) -> GdeResult<()>;
}
