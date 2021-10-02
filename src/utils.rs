use std::path::PathBuf;
use crate::error::GdeError;
use lazy_static::lazy_static;
use std::process::Command;
use std::ffi::OsStr;

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
            pb = std::env::current_exe().expect("Failed to get path").parent().unwrap().to_owned();
        }
        println!("LIB_PATH: {}", pb.display());
        pb
    };

    pub static ref CACHE_PATH: PathBuf = std::env::current_dir().expect("Failed to get path").join("cache");

    pub static ref STD_MACRO_PATH: PathBuf = {
        let mut pb = (&*LIB_PATH).to_owned();
        pb.push("default.r4f");
        pb.to_path_buf()
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
        pb.push("renderers");
        pb
    };

    pub static ref DEFAULT_ENTRY_PATH: PathBuf = std::env::current_dir().expect("Failed to get path").join("index.gddt");

    pub static ref CONFIG_PATH: PathBuf = std::env::current_dir().expect("Failed to get path").join("config.json");
}

pub fn module_path(name : impl AsRef<str>) -> Result<PathBuf, GdeError> {
    Ok(LIB_PATH.join(format!("{}.r4f", name.as_ref())).to_owned())
}

pub fn renderer_path(name : impl AsRef<str>) -> Result<PathBuf, GdeError> {
    Ok(RENDERER_PATH.join(name.as_ref()))
}

pub fn middle_file_path() -> Result<PathBuf, GdeError> {
    Ok(CACHE_PATH.join("out.gddt"))
}

// Cross platform command call
pub fn command(program: &str, args: Vec<impl AsRef<OsStr>>) -> Result<(), GdeError> {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(program)
            .args(args)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(program)
            .args(args)
            .output()
            .expect("failed to execute process")
    };
    let out_content = String::from_utf8_lossy(&output.stdout);
    let err_content = String::from_utf8_lossy(&output.stderr);
    if out_content.len() != 0 {println!("{}", out_content);}
    if err_content.len() != 0 {eprintln!("{}", err_content);}

    Ok(())
}
