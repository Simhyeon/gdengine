use std::fs::canonicalize;

use r4d::rad_ext_template::*;
use r4d::{ExtMacroBuilder, MacroType, Processor, RadError, RadResult};

pub struct Extension;

impl Extension {
    pub fn extend_all(processor: &mut Processor) {
        Self::extend_ifmod(processor);
        Self::extend_image_size(processor);
    }

    fn extend_ifmod(processor: &mut Processor) {
        // Add if mod variant
        processor.add_ext_macro(
            ExtMacroBuilder::new("ifmod")
            .args(&["a_mod","a_content"])
            .deterred(deterred_template!(
                    let args = split_args!(2)?;
                    let target_mod = format!("mod_{}", &args[0]);

                    let result: Option<String> = if processor.contains_macro(&target_mod,MacroType::Any) {
                        Some(expand!(&args[1])?)
                    } else { None };

                    Ok(result)
            ))
        );
        processor.add_ext_macro(
            ExtMacroBuilder::new("ifmodel")
            .args(&["a_mod","a_content","a_content_else"])
            .deterred(deterred_template!(
                    let args = split_args!(3)?;
                    let target_mod = format!("mod_{}", &args[0]);

                    let result: Option<String> = if processor.contains_macro(&target_mod,MacroType::Any) {
                        Some(expand!(&args[1])?)
                    } else {
                        Some(expand!(&args[2])?)
                    };

                    Ok(result)
            ))
        );
    }

    fn extend_image_size(processor: &mut Processor) {
        processor.add_ext_macro(
            ExtMacroBuilder::new("image_dim")
            .args(&["a_path"])
            .function(function_template!(
                    let args = split_args!(1)?;
                    let path = canonicalize(processor.get_current_dir()?.join(&args[0]))?;

                    let dim = imagesize::size(&path).map_err(|err| {
                        RadError::InvalidExecution(format!("Failed to get image size of \"{}\" with error of \"{}\"",path.display(),err))
                    })?;

                    let string = format!("{},{}",dim.width,dim.height);
                    Ok(Some(string))
            ))
        );

        processor.add_ext_macro(
            ExtMacroBuilder::new("image_height")
            .args(&["a_path"])
            .function(function_template!(
                    let args = split_args!(1)?;
                    let path = canonicalize(processor.get_current_dir()?.join(&args[0]))?;
                    let dim = imagesize::size(&path).map_err(|err| {
                        RadError::InvalidExecution(format!("Failed to get image size of \"{}\" with error of \"{}\"",path.display(),err))
                    })?;
                    Ok(Some(format!("{}",dim.height)))
            ))
        );

        processor.add_ext_macro(
            ExtMacroBuilder::new("image_width")
            .args(&["a_path"])
            .function(function_template!(
                    let args = split_args!(1)?;
                    let path = canonicalize(processor.get_current_dir()?.join(&args[0]))?;
                    let dim = imagesize::size(&path).map_err(|err| {
                        RadError::InvalidExecution(format!("Failed to get image size of \"{}\" with error of \"{}\"",path.display(),err))
                    })?;
                    Ok(Some(format!("{}",dim.width)))
            ))
        );
    }
}
