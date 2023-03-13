
/*
    This program can be run with the command: `cargo run`

    Note that it requires two arguments, a _pattern_ and a _path_.

    An example that runs cleanly: `cargo run -- foo .`
 */

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("pattern: {}, path: {}", args.pattern, args.path.display());
}
