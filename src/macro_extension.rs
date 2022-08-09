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
            .desc("If current mode is given value then execute expression")
            .deterred(deterred_template!(
                    let args = split_args!(2, true)?;
                    let target_mod = format!("mod_{}", &args[0]);

                    let result: Option<String> = if processor.contains_macro(&target_mod,MacroType::Any) {
                        Some(expand_args!(&args[1])?)
                    } else { None };

                    Ok(result)
            ))
        );
        processor.add_ext_macro(
            ExtMacroBuilder::new("ifmodel")
            .args(&["a_mod","a_content","a_content_else"])
            .desc("If current mode is given value then execute expression else the other")
            .deterred(deterred_template!(
                    let args = split_args!(3,true)?;
                    let target_mod = format!("mod_{}", &args[0]);

                    let result: Option<String> = if processor.contains_macro(&target_mod,MacroType::Any) {
                        Some(expand_args!(&args[1])?)
                    } else {
                        Some(expand_args!(&args[2])?)
                    };

                    Ok(result)
            ))
        );
    }

    // TODO
    // std::env::current_dir() might not work.
    fn extend_image_size(processor: &mut Processor) {
        processor.add_ext_macro(
            ExtMacroBuilder::new("image_dim")
            .args(&["a_path"])
            .desc("Get image size from a given path")
            .function(function_template!(
                    let args = split_args!(1,false)?;
                    let path = canonicalize(std::env::current_dir().expect("Failed to get current directory").join(&args[0]))?;

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
            .desc("Get image height from a given path")
            .function(function_template!(
                    let args = split_args!(1, false)?;
                    let path = canonicalize(std::env::current_dir().expect("Failed to get current directory").join(&args[0]))?;
                    let dim = imagesize::size(&path).map_err(|err| {
                        RadError::InvalidExecution(format!("Failed to get image size of \"{}\" with error of \"{}\"",path.display(),err))
                    })?;
                    Ok(Some(format!("{}",dim.height)))
            ))
        );

        processor.add_ext_macro(
            ExtMacroBuilder::new("image_width")
            .args(&["a_path"])
            .desc("Get image width from a given path")
            .function(function_template!(
                    let args = split_args!(1, false)?;
                    let path = canonicalize(std::env::current_dir().expect("Failed to get current directory").join(&args[0]))?;
                    let dim = imagesize::size(&path).map_err(|err| {
                        RadError::InvalidExecution(format!("Failed to get image size of \"{}\" with error of \"{}\"",path.display(),err))
                    })?;
                    Ok(Some(format!("{}",dim.width)))
            ))
        );
    }
}
