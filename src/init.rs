use std::fs;
use std::path::Path;
use crate::error::GdeError;
use std::process::Command;

const DIRS: [&str; 4] = [ "inc", "build", "cache", "res" ];

pub struct Init;

impl Init {
    pub fn new_gddt_file() -> Result<(), GdeError> {
        fs::write(Path::new("index.gddt"), "")?;
        Ok(())
    }

    // Create new macro file
    pub fn new_macro_file() -> Result<(), GdeError> {
        Init::m4_macro_file()?;
        Ok(())
    }

    // Crate new m4 macro file
    fn m4_macro_file() -> Result<(), GdeError> {
        // Index m4 file
        fs::write(Path::new("index.m4"), "divert(`-1')\n\ndivert`'")?;
        // env file
        fs::write(Path::new("env.m4"), "divert(`-1')\n\ndivert`'")?;
        Ok(())
    }

    // TODO 
    // Complete this when rad is complete
    fn rad_macro_file() -> Result<(), GdeError> {
        unimplemented!();
        Ok(())
    }

    pub fn git_init() -> Result<(), GdeError> {
        // Git init
        Command::new("git")
            .args(["init"])
            .output()?;
        // Crate gitignore file
        fs::write(Path::new(".gitignore"), "build\ncache")?;
        Ok(())
    }

    pub fn new_directories() -> Result<(), GdeError> {
        for dir in DIRS {
            fs::create_dir(Path::new(dir))?;
        }
        Ok(())
    }
} 
