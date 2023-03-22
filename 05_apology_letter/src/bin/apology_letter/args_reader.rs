use clap::Parser;

/// Searches for a pattern in a file and displays the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    pub recipient_name: String,
    pub thing_you_did: String,
}

pub fn read_args() -> Cli {
    Cli::parse()
}