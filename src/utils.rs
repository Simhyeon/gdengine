use std::path::{PathBuf};
use crate::error::GdeError;
use lazy_static::lazy_static;

// Paths
lazy_static! {
    pub static ref BUILD_PATH: PathBuf = {
        let mut pb;
        if cfg!(debug_assertions) {
            pb = std::env::current_dir().expect("Failed to get path");
        } else {
            pb = std::env::current_exe().expect("Failed to get path");
        }
        pb.push("build");
        pb
    };

    pub static ref LIB_PATH: PathBuf = {
        let mut pb;
        if cfg!(debug_assertions) {
            pb = std::env::current_dir().expect("Failed to get path");
        } else {
            pb = std::env::current_exe().expect("Failed to get path");
        }
        pb.push("libs");
        pb
    };

    pub static ref CACHE_PATH: PathBuf = {
        let mut pb;
        if cfg!(debug_assertions) {
            pb = std::env::current_dir().expect("Failed to get path");
        } else {
            pb = std::env::current_exe().expect("Failed to get path");
        }
        pb.push("cache");
        pb
    };

    pub static ref STD_MACRO_PATH: PathBuf = {
        let mut pb;
        if cfg!(debug_assertions) {
            pb = std::env::current_dir().expect("Failed to get path");
        } else {
            pb = std::env::current_exe().expect("Failed to get path");
        }
        pb.push("libs");
        pb.push("default.r4f");
        pb
    };

    // This itself is not used outside of utils file
    // This is used by renderer_path method which puts renderer name at the end
    static ref RENDERER_PATH: PathBuf = {
        let mut pb;
        if cfg!(debug_assertions) {
            pb = std::env::current_dir().expect("Failed to get path");
        } else {
            pb = std::env::current_exe().expect("Failed to get path");
        }
        pb.push("renderer");
        pb
    };

    pub static ref DEFAULT_ENTRY_PATH: PathBuf = {
        std::env::current_dir().expect("Failed to get path").join("index.gddt")
    };

    pub static ref CONFIG_PATH: PathBuf = {
        std::env::current_dir().expect("Failed to get path").join("config.json")
    };
}

pub fn module_path(name : &str) -> Result<PathBuf, GdeError> {
    Ok(LIB_PATH.join(format!("{}.r4f", name)).to_owned())
}

pub fn renderer_path(name : &str) -> Result<PathBuf, GdeError> {
    Ok(RENDERER_PATH.join("name"))
}

pub fn middle_file_path() -> Result<PathBuf, GdeError> {
    Ok(CACHE_PATH.join("out.gddt"))
}
