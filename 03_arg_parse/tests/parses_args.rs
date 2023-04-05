use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn works_when_passed_two_args() -> Result<(), Box<dyn std::error::Error>> {
    let arg1 = "foo";
    let arg2 = "bar";

    let output_bytes = Command::cargo_bin("arg_parse")?
        .args([arg1])
        .args([arg2])
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected = format!("pattern: {}, path: {}\n", arg1, arg2);

    assert_eq!(output_str, expected);

    Ok(())
}

#[test]
fn doesnt_work_when_passed_zero_or_one_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("arg_parse")?;

    cmd.assert().failure();

    cmd.arg("foor");
    cmd.assert().failure();

    Ok(())
}
