
/*
    This program can be run with the command: `cargo run`

    Note that it requires two arguments, a _pattern_ and a _path_.

    An examle that runs cleanly: `cargo run -- foo .`
 */

fn main() {

    #[derive(Debug)]
    struct Cli {
        pattern: String,
        path: std::path::PathBuf,
    }

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("{:#?}", args);
}
