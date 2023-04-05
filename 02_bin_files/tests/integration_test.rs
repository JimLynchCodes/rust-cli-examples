use std::str;
use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn prints_3() -> Result<(), Box<dyn std::error::Error>> {

    let output_bytes = Command::cargo_bin("bin_folder")?.output().unwrap().stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert_eq!(output_str, "3\n");

    Ok(())
}