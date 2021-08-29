mod cli;
mod config;
mod error;
mod init;
mod executor;
mod renderer;
mod utils;

use cli::Cli;

fn main() {
    let result = Cli::parse();
    if let Some(err) = result.err() {
        eprintln!("{}", err);
    }
}
