use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
/// Note how this integration test actually makes the api calls
fn gets_and_prints_json() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("get_json")?.output().unwrap().stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert!(output_str.contains("username"));

    Ok(())
}
