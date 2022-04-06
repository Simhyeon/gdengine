mod cli;
mod config;
mod error;
mod executor;
mod macro_extension;
mod init;
mod models;
mod orchestrator;
mod renderer;
mod utils;

use cli::{Cli, Variant};
extern crate dotenv;
use dotenv::dotenv;

fn main() {
    // Use dotenv file's variables
    dotenv().ok();

    let result = Cli::new(Variant::None).parse();
    if let Some(err) = result.err() {
        eprintln!("{}", err);
    }
}
