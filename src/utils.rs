use std::path::PathBuf;
use crate::error::GdeError;

pub fn build_path() -> Result<PathBuf, GdeError> {
    let mut pb = std::env::current_dir()?;
    pb.push("build");
    Ok(pb)
}

pub fn cache_path() -> Result<PathBuf, GdeError> {
    let mut pb = std::env::current_dir()?;
    pb.push("cache");
    Ok(pb)
}

pub fn m4_gnu_path() -> Result<PathBuf, GdeError> {
    let mut pb = std::env::current_exe()?;
    pb.push("m4");
    pb.push("GNU");
    Ok(pb)
}

pub fn m4_std_path() -> Result<PathBuf, GdeError> {
    let mut pb = std::env::current_exe()?;
    pb.push("m4");
    pb.push("default.m4");
    Ok(pb)
}

pub fn module_path() -> Result<PathBuf, GdeError> {
    let mut pb = std::env::current_exe()?;
    pb.push("module");

    Ok(pb)
}

pub fn script_path() -> Result<PathBuf, GdeError> {
    let mut pb = std::env::current_exe()?;
    pb.push("m4");
    pb.push("scripts");

    Ok(pb)
}
