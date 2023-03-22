/*
   This program can be run with the command: `cargo run`

   Note that it requires two arguments, a _pattern_ and a _path_.

   An example that runs cleanly: `cargo run -- foo .`
*/

use clap::Parser;

/// Searches for a pattern in a file and displays the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("pattern: {}, path: {}", args.pattern, args.path.display());
}