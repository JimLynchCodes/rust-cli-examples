/// Parse the command line arguments
/// 
/// Expecting 1 optional positional argument: number of letters for word, default value is: 5
/// 
/// Expecting 1 optional flag: --guesses or -g, which takes an integer parameter, default value is: 6
/// 
/// 

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(default_value_t = 5)]
    pub letters_in_word: u8,

    #[arg(short, long, default_value_t = 6)]
    pub guesses: u8,
}

pub fn read_args() -> Cli {
    Cli::parse()
}

