use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::error::GdeError;
use crate::utils;

pub struct RenderOptions {
    preserve: bool,
    copy: Option<PathBuf>,
    format: Option<String>,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl RenderOptions {
    pub fn new(preserve:bool, copy:Option<PathBuf>, format: Option<String>, input: Option<PathBuf>, output: Option<PathBuf>) -> Self {
        Self { 
            preserve,
            copy,
            format,
            input,
            output,
        }
    }
}

pub struct Renderer<'a> {
    target: &'a str,
    module: &'a str,
    options : RenderOptions,
}

impl<'a> Renderer<'a> {
    pub fn new(target: &'a str, module: &'a str, options: RenderOptions) -> Self {
        Self { 
            target,
            module,
            options,
        }
    }

    pub fn render(&self) -> Result<(), GdeError> {
        // Render with m4
        self.m4_render()?;
        Ok(())
    }

    fn m4_render(&self) -> Result<(), GdeError> {

        self.m4_preprocess()?;
        self.m4_render_with_module()?;
        self.m4_postprocess()?;

        Ok(())
    }

    fn m4_render_with_module(&self) -> Result<(), GdeError> {
        Ok(())
    }

    fn m4_preprocess(&self) -> Result<(), GdeError> {
        let args = self.m4_arguments()?;

        let mut frag_a_file = std::env::current_dir()?;
        frag_a_file.push("frag_a.gddt");
        let mut frag_b_file = std::env::current_dir()?;
        frag_b_file.push("frag_b.gddt");

        let output = Command::new("m4")
            .args(&args)
            .output()?;

        // Write once
        fs::write(
            frag_a_file, 
            String::from_utf8_lossy(&output.stdout).to_string()
        )?;
        println!("{}", String::from_utf8_lossy(&output.stderr));

        let output = Command::new("m4")
            .args(&args)
            .output()?;

        // Write twice but to b this time
        fs::write(
            frag_b_file, 
            String::from_utf8_lossy(&output.stdout).to_string()
        )?;
        println!("{}", String::from_utf8_lossy(&output.stderr));

        Ok(())
    }

    fn m4_postprocess(&self) -> Result<(), GdeError> {
        Ok(())
    }

    fn m4_arguments(&self) -> Result<Vec<String>, GdeError> {
        let include = self.m4_include_dirs()?;

        let mut include_args: Vec<String> = vec![];
        for dir in &include {
            include_args.push("-I".to_owned());
            include_args.push(dir.to_str().unwrap().to_owned());
        }

        let mut sources = self.m4_add_sources()?
            .into_iter()
            .map(|path| path.to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        // Mnemoic naming for easier readability
        let mut args = include_args;
        args.append(&mut sources);

        // Return vec
        Ok(args)
    }

    fn m4_include_dirs(&self) -> Result<Vec<PathBuf>, GdeError> {
        // TODO
        // User defined include path
        let mut sources = vec!(
            utils::m4_gnu_path()?, 
            std::env::current_dir()?,
        );

        Ok(sources)
    }

    fn m4_add_sources(&self) -> Result<Vec<PathBuf>, GdeError> {
        // Default sources
        let mut sources = vec!(
            utils::m4_std_path()?, 
            PathBuf::from("env.m4"), 
            PathBuf::from("index.m4"),
            PathBuf::from("index.gddt"),
        );

        // Set root of the module path
        let mut root_module_path = utils::module_path()?;
        root_module_path.push(self.target);
        root_module_path.push(self.module);

        // Set root of the macro and env file's path
        let mut macro_path = root_module_path.clone();
        let mut env_path = root_module_path.clone();
        macro_path.push("macro.m4");
        env_path.push("env.m4");

        // Add sources
        sources.push(macro_path.clone());
        sources.push(env_path.clone());

        Ok(sources)
    }
}
