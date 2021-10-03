use std::fs;
use std::path::Path;
use crate::config::Config;
use crate::error::GdeError;
use crate::utils;
use std::ffi::OsStr;

const DIRS: [&str; 4] = [ "inc", "build", "cache", "res" ];

pub struct Init;

impl Init {
    pub fn new_gddt_file() -> Result<(), GdeError> {
        fs::write(Path::new("index.gddt"), "")?;
        Ok(())
    }
    pub fn new_config_file() -> Result<(), GdeError> {
        fs::write(Path::new("config.json"), Config::new_file())?;
        Ok(())
    }

    // Create new macro file
    pub fn new_macro_file() -> Result<(), GdeError> {
        Init::macro_file()?;
        Ok(())
    }

    // Crate new m4 macro file
    fn macro_file() -> Result<(), GdeError> {
        // Index m4 file
        fs::write(Path::new("index.r4d"), "")?;
        Ok(())
    }

    pub fn git_init() -> Result<(), GdeError> {
        // Git init
        utils::command("git",vec![OsStr::new("init")])?;

        // Crate gitignore file
        fs::write(Path::new(".gitignore"), "build\ncache\nres")?;
        Ok(())
    }

    pub fn new_directories() -> Result<(), GdeError> {
        for dir in DIRS {
            fs::create_dir(Path::new(dir))?;
        }
        Ok(())
    }
} 
