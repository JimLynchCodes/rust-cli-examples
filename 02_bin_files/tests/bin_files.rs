use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn prints_6() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("bin_files")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert_eq!(output_str, "6\n");

    Ok(())
}
