//! Welcome to the Hello, World project!
//!
//! Notice the comments with an extra exclamation mark. This
//! allows us to create idiomatic Rust module or crate level documentation.
//! - Bullet point
//! - [`Some Link`]
//!  This program can be run with the command: `cargo run`
//!
//!  This cli tool requires two arguments, a _pattern_ and a _path_.
//!
//!  Note that the pattern argument can be any string but the path must be a valid file or folder path.
//!
//!  An example that runs cleanly: `cargo run -- foo .`
//!
//! [`Link Example`]: http://thatwaseasy.example.com

/*
   This program can be run with the command: `cargo run`

   Note that it requires two arguments, a _pattern_ and a _path_.

   The

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
