use clap::{arg, Command};

pub struct BuilderArgs {
    pub arg1: String,
    pub arg2: Option<String>,
    pub flag: Option<u32>,
}

pub const ARG_1_NAME: &str = "arg1";
pub const ARG_2_NAME: &str = "arg2";
pub const FLAG_NAME: &str = "flag";

pub fn read_args() -> BuilderArgs {
    let matches = Command::new("MyApp")
        .arg(arg!([arg1]).required(true))
        .arg(arg!([arg2]).required(false))
        .arg(arg!(-f --flag <VALUE>).required(false))
        .get_matches();

    BuilderArgs {
        arg1: matches
            .get_one::<String>(ARG_1_NAME)
            .expect("required")
            .to_string(),
        arg2: match matches.get_one::<String>(ARG_2_NAME) {
            Some(arg2) => Some(arg2.to_string()),
            None => None,
        },
        flag: match matches.get_one::<String>(FLAG_NAME) {
            Some(flag) => Some(
                flag.parse::<u32>()
                    .expect("Couldn't parse flag value to u32"),
            ),
            None => None,
        },
    }
}
