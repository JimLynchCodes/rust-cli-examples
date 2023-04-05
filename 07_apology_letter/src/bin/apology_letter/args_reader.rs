use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub recipient_name: String,
    pub thing_you_did: String,
}

pub fn read_args() -> Cli {
    Cli::parse()
}
