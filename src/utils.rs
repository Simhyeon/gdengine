use std::path::PathBuf;
use crate::error::GdeError;
use lazy_static::lazy_static;

// TODO 
// Make this lazy_static rather tahn creating pathbuf everytime

pub fn build_path() -> Result<PathBuf, GdeError> {
    let mut pb;
    if cfg!(debug_assertions) {
        pb = std::env::current_dir()?;
    } else {
        pb = std::env::current_exe()?;
    }
    pb.push("build");
    Ok(pb)
}

pub fn lib_path() -> Result<PathBuf, GdeError> {
    let mut pb;
    if cfg!(debug_assertions) {
        pb = std::env::current_dir()?;
    } else {
        pb = std::env::current_exe()?;
    }
    pb.push("libs");
    Ok(pb)
}

pub fn cache_path() -> Result<PathBuf, GdeError> {
    let mut pb;
    if cfg!(debug_assertions) {
        pb = std::env::current_dir()?;
    } else {
        pb = std::env::current_exe()?;
    }
    pb.push("cache");
    Ok(pb)
}

pub fn module_path(name : &str) -> Result<PathBuf, GdeError> {
    Ok(lib_path()?.join(format!("{}.r4f", name)).to_owned())
}

pub fn std_path() -> Result<PathBuf, GdeError> {
    let mut pb;
    if cfg!(debug_assertions) {
        pb = std::env::current_dir()?;
    } else {
        pb = std::env::current_exe()?;
    }
    pb.push("libs");
    pb.push("default.r4f");
    Ok(pb)
}

pub fn renderer_path() -> Result<PathBuf, GdeError> {
    let mut pb;
    if cfg!(debug_assertions) {
        pb = std::env::current_dir()?;
    } else {
        pb = std::env::current_exe()?;
    }
    pb.push("renderers");
    Ok(pb)
}

pub fn middle_file_path() -> Result<PathBuf, GdeError> {
    let out_file = cache_path()?
        .join("out.gddt");

    Ok(out_file)
}

pub fn default_entry_path() -> Result<PathBuf, GdeError> {
    let pb = std::env::current_dir()?.join("index.gddt");
    Ok(pb)
}
