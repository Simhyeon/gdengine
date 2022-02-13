use std::fs;
use std::path::Path;
use crate::config::Config;
use crate::models::GdeResult;
use crate::utils;
use std::ffi::OsStr;

const DIRS: [&str; 4] = [ "inc", "build", "cache", "res" ];

pub struct Init;

impl Init {
    pub fn new_gddt_file() -> GdeResult<()> {
        fs::write(Path::new("index.gddt"), "")?;
        Ok(())
    }
    pub fn new_rad_file() -> GdeResult<()> {
        fs::write(&*utils::INDEX_RAD, "")?;
        Ok(())
    }
    pub fn new_config_file() -> GdeResult<()> {
        fs::write(Path::new("gde_config.json"), Config::new_file())?;
        Ok(())
    }

    pub fn new_env_file() -> GdeResult<()> {
        fs::write(Path::new(".env"), "")?;
        Ok(())
    }

    pub fn new_var_file() -> GdeResult<()> {
        fs::write(Path::new("varfile.env"), "")?;
        Ok(())
    }

    // Create new macro file
    pub fn new_macro_file() -> GdeResult<()> {
        Init::macro_file()?;
        Ok(())
    }

    // Crate new m4 macro file
    fn macro_file() -> GdeResult<()> {
        // Index m4 file
        fs::write(Path::new("index.r4d"), "")?;
        Ok(())
    }

    pub fn git_init() -> GdeResult<()> {
        // Git init
        utils::command("git",vec![OsStr::new("init")])?;

        // Crate gitignore file
        fs::write(Path::new(".gitignore"), "build\ncache\nres\n.env")?;
        Ok(())
    }

    pub fn new_directories() -> GdeResult<()> {
        for dir in DIRS {
            fs::create_dir(Path::new(dir))?;
        }
        Ok(())
    }
} 
