use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn works_when_passed_two_args() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("env_reader")?.output().unwrap().stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected = "value for the env var MY_VAR_1 is: hello!!\nvalue for the env var MY_VAR_2 is: 44\nvalue for the env var MY_VAR_3 is: foo\n";

    assert_eq!(output_str, expected);

    Ok(())
}
