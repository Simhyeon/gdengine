mod cli;
mod config;
mod error;
mod init;
mod orchestrator;
mod executor;
mod renderer;
mod utils;

use cli::{Cli, Variant};

fn main() {
    let result = Cli::new(Variant::None).parse();
    if let Some(err) = result.err() {
        eprintln!("{}", err);
    }
}
