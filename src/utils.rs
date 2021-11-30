use std::path::{PathBuf, Path};
use crate::models::GdeResult;
use lazy_static::lazy_static;
use regex::Regex;
use std::process::Command;
use std::ffi::OsStr;

const REG_CHOMP_REPL: &str = "\n\n";

// Paths
lazy_static! {
    pub static ref BUILD_PATH: PathBuf = std::env::current_dir().expect("Failed to get path").join("build");

    static ref CRLF_MATCH : Regex = Regex::new(r#"\r\n"#).unwrap();
    static ref REG_CHOMP_MATCH : Regex = Regex::new(r#"\n\s*\n"#).expect("Failed to crate chomp regex");

    pub static ref LIB_PATH: PathBuf = {
        let mut pb;
        if cfg!(debug_assertions) {
            pb = std::env::current_dir().expect("Failed to get path");
        } else {
            pb = std::env::current_exe().expect("Failed to get path").parent().unwrap().to_owned();
        }
        pb.push("libs");
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
            pb = std::env::current_exe().expect("Failed to get path").parent().expect("Failed to get path").to_path_buf();
        }
        pb.push("renderers");
        pb
    };

    pub static ref DEFAULT_ENTRY_PATH: PathBuf = std::env::current_dir().expect("Failed to get path").join("index.gddt");

    pub static ref CONFIG_PATH: PathBuf = std::env::current_dir().expect("Failed to get path").join("gde_config.json");

    pub static ref INDEX_RAD: PathBuf = std::env::current_dir().expect("Failed to get path").join("index.r4d");
}

pub fn module_path(name : impl AsRef<str>) -> GdeResult<PathBuf> {
    Ok(LIB_PATH.join(format!("{}.r4f", name.as_ref())).to_owned())
}

pub fn renderer_path(name : impl AsRef<str>) -> GdeResult<PathBuf> {
    Ok(RENDERER_PATH.join(name.as_ref()))
}

/// out.gddt
pub fn middle_file_path() -> GdeResult<PathBuf> {
    Ok(CACHE_PATH.join("out.gddt"))
}

// Chomp file save contents into memory in a belief that file sizes would not be that big...
// I mean come on, every file is a human readable text file. It should not be gigabytes
// sized
pub fn chomp_file(path: &Path) -> GdeResult<()> {
    let content = &std::fs::read_to_string(path)?;
    let sanitized = CRLF_MATCH.replace(content, r#"\n"#);
    let replaced = REG_CHOMP_MATCH.replace(&sanitized, REG_CHOMP_REPL);
    std::fs::write(path, replaced.as_bytes())?;
    Ok(())
}

// Cross platform command call
pub fn command(program: &str, args: Vec<impl AsRef<OsStr>>) -> GdeResult<()> {

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
