
#[derive(Debug)]
pub struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

pub fn get() -> Cli {

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    args
}

fn main() {
    assert_eq!(process(), 100);
}

#[test]
fn correct_value() {
    assert_eq!(process(), 42u8);
}

fn process() -> u8 {
    do_it()
}

#[cfg(not(test))]
fn do_it() -> u8 {
    100 // *real* process result
}

#[cfg(test)]
fn
