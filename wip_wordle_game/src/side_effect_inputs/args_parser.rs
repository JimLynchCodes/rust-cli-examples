use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(default_value_t = 5)]
    pub letters_in_word: u8,

    #[arg(short, long, default_value_t = 6)]
    pub guesses: u8,

    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

pub fn read_args() -> Cli {
    Cli::parse()
}
