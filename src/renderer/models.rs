use std::path::PathBuf;

use crate::{executor::ExecOption, models::GdeResult};
use r4d::Processor;

// Game design document renderer trait
pub(crate) trait GRender {
    fn render(&self, processor: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>>;
    fn rad_setup(&self, processor: &mut Processor) -> GdeResult<()>;
}
