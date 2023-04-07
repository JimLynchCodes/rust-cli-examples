use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub arg1: String,
    pub arg2: Option<String>,

    #[arg(short, long)]
    pub flag: Option<u32>,
}

pub fn read_args() -> Cli {
    Cli::parse()
}
