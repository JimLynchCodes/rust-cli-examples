use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn prints_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("hello_world")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert_eq!(output_str, "Hello, world!\n");

    Ok(())
}
