use std::path::{PathBuf, Path};
use crate::error::GdeError;
use crate::models::GdeResult;
use lazy_static::lazy_static;
use regex::Regex;
use std::process::{Command, Stdio};
use std::ffi::OsStr;
use std::io::Write;

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

// TODO 
// Add exe at the end if windows
// Check debug features
pub fn renderer_bin_path(renderer: impl AsRef<str>, bin_path: impl AsRef<Path>) -> GdeResult<PathBuf> {
    let mut static_bin : PathBuf;

    // If binary already exists in environment paths, simply use it.
    // If not, use statically contained binary
    static_bin = match which::which(bin_path.as_ref().file_name().unwrap()) {
        Ok(path) => path,
        Err(_) => {
            let static_bin = RENDERER_PATH
                .join(renderer.as_ref())
                .join("bin") // This is fixed path
                .join(bin_path.as_ref());

            // File should exists
            if !static_bin.exists() {
                return Err(GdeError::NoSuchPath(format!("Renderer \"{}\" doesn't exist.", static_bin.display())));
            }

            static_bin
        },
    };

    // Set extension for windows version
    if cfg!(target_os = "windows") {
        static_bin.set_extension("exe");
    }
    Ok(static_bin)
}

/// out.gddt
pub fn middle_file_path() -> GdeResult<PathBuf> {
    Ok(CACHE_PATH.join("out.gddt"))
}

/// out.gddt
pub fn middle_file_path_with_toc() -> GdeResult<PathBuf> {
    Ok(CACHE_PATH.join("out_with_toc.gddt"))
}

// Chomp file save contents into memory in a belief that file sizes would not be that big...
// I mean come on, every file is a human readable text file. It should not be gigabytes
// sized
pub fn chomp_file(path: &Path) -> GdeResult<()> {
    let content = &std::fs::read_to_string(path)?;
    let sanitized = CRLF_MATCH.replace_all(content, "\n");
    let replaced = REG_CHOMP_MATCH.replace_all(&sanitized, REG_CHOMP_REPL);
    std::fs::write(path, replaced.as_bytes())?;
    Ok(())
}

// Cross platform command call
pub fn command(program: &str, args: Vec<impl AsRef<OsStr>>) -> GdeResult<()> {
    command_logic(program,args,None)
}

// Cross platform command call with stdin
pub fn command_with_stdin(program: &str, args: Vec<impl AsRef<OsStr>>, input_value: &str) -> GdeResult<()> {
    command_logic(program,args,Some(input_value))
}

/// Real logic method of command execution
fn command_logic(program: &str, args: Vec<impl AsRef<OsStr>>, stdin: Option<&str>) -> GdeResult<()> {

    let stdin_type = if let Some(_) = stdin {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };
    let mut process = if cfg!(target_os = "windows") { // Windows
        Command::new("cmd")
            .arg("/C")
            .arg(program)
            .args(args)
            .stdin(stdin_type)
            .spawn()
            .expect("failed to execute process")
    } else { // Nix based
        Command::new(program)
            .stdin(stdin_type)
            .args(args)
            .spawn()
            .expect("failed to execute process")
    };

    if let Some(input) = stdin {
        let input = input.to_string();
        let mut stdin = process.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
        });
    }
    let output = process.wait_with_output().expect("Failed to read stdout");

    let out_content = String::from_utf8_lossy(&output.stdout);
    let err_content = String::from_utf8_lossy(&output.stderr);

    if out_content.len() != 0 {writeln!(std::io::stdout(),"{}", out_content)?;}
    if err_content.len() != 0 {writeln!(std::io::stderr(),"{}", err_content)?;}

    Ok(())
}
