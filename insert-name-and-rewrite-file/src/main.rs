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

const recipient_name_placeholder: &str = "*[__RECIPIENT_NAME__]*";
const thing_you_did_placeholder: &str = "*[__THING_YOU_DID__]*";

fn main() {
    let args = Cli::parse();
    println!("pattern: {}, path: {}", args.pattern, args.path.display());

    let content = std::fs::read_to_string("./example.txt").expect("could not read file");
    println!("read: {}", content);

    for line in content.lines() {
        if line.contains(&recipient_name_placeholder) {
            println!("name line! {}", line);
        } else if line.contains(&thing_you_did_placeholder) {
            println!("thing you did line! {}", line);
        } else {
            println!("regular line: {}", line);
        }
    }
}
