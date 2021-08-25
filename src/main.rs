mod cli;
mod error;
mod init;
mod render;
mod renderer;
mod utils;

use cli::Cli;

fn main() {
    let result = Cli::parse();
    if let Some(err) = result.err() {
        println!("{}", err);
    }
}
