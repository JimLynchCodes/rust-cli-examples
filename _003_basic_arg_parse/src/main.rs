
/*
    This program can be run with the command: `cargo run`

    Note that it requires two arguments, a _pattern_ and a _path_.

    An example that runs cleanly: `cargo run -- foo .`

    The integration test (found in the /test folder) can be run with `cargo test`.

    Note that there is no unit test here because here in this contrived example all ofthe logic is right in the main function (not reccommended, in general).
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
