use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
/// TODO - come back to this after we figure out how to test inquire things 😅
/// follow along in the issue here: https://github.com/mikaelmello/inquire/issues/71
fn always_responds_empty_string() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("happy_birthday")?.output()?.stdout;

    let output_str = str::from_utf8(&output_bytes)?;

    assert_eq!(output_str, "");

    Ok(())
}
