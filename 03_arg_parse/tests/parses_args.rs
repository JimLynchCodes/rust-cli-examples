use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn works_when_passed_two_args() -> Result<(), Box<dyn std::error::Error>> {
    let arg1 = "foo";
    let arg2 = "bar";

    let output_bytes = Command::cargo_bin("arg_parse")?
        .args([arg1, arg2])
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected = format!("arg1: {arg1}, arg2: Some(\"{arg2}\"), flag: None\narg1: {arg1}, arg2: Some(\"{arg2}\"), flag: None\n");

    assert_eq!(output_str, expected);

    Ok(())
}

#[test]
fn works_when_passed_one_arg() -> Result<(), Box<dyn std::error::Error>> {
    let arg1 = "foo";

    let output_bytes = Command::cargo_bin("arg_parse")?
        .args([arg1])
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected =
        format!("arg1: {arg1}, arg2: None, flag: None\narg1: {arg1}, arg2: None, flag: None\n");

    assert_eq!(output_str, expected);

    Ok(())
}

#[test]
#[ignore]
fn works_when_passed_one_arg_and_short_flag() -> Result<(), Box<dyn std::error::Error>> {
    let arg1 = "foo";

    let output_bytes = Command::cargo_bin("arg_parse")?
        .args([arg1, "-f", "42"])
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected = "arg1: foo, arg2: None, flag: Some(42)\narg1: foo, arg2: None, flag: Some(42)\n";

    assert_eq!(output_str, expected);

    Ok(())
}

#[test]
fn works_when_passed_two_args_and_long_flag() -> Result<(), Box<dyn std::error::Error>> {
    let arg1 = "foo";
    let arg2 = "bar";
    let flag = 42;

    let output_bytes = Command::cargo_bin("arg_parse")?
        .args([arg1, arg2, "--flag", &flag.to_string()])
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected = format!("arg1: {arg1}, arg2: Some(\"{arg2}\"), flag: Some({flag})\narg1: {arg1}, arg2: Some(\"{arg2}\"), flag: Some({flag})\n");

    assert_eq!(output_str, expected);

    Ok(())
}

#[test]
fn doesnt_work_when_passed_zero_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("arg_parse")?;

    cmd.assert().failure();

    Ok(())
}

#[test]
fn doesnt_work_when_passing_short_flag_with_no_param() -> Result<(), Box<dyn std::error::Error>> {
    let arg1 = "foo";
    let arg2 = "bar";

    let output_err = Command::cargo_bin("arg_parse")?
        .args([arg1, arg2, "-f"])
        .output()
        .unwrap()
        .stderr;

    let expected = "error: a value is required for '--flag <FLAG>' but none was supplied\n\nFor more information, try '--help'.\n";

    assert_eq!(str::from_utf8(&output_err).unwrap(), expected);

    Ok(())
}

#[test]
fn doesnt_work_when_passing_long_flag_with_no_param() -> Result<(), Box<dyn std::error::Error>> {
    let output_err = Command::cargo_bin("arg_parse")?
        .args(["--flag"])
        .output()
        .unwrap()
        .stderr;

    let expected = "error: a value is required for '--flag <FLAG>' but none was supplied\n\nFor more information, try '--help'.\n";

    assert_eq!(str::from_utf8(&output_err).unwrap(), expected);

    Ok(())
}

#[test]
fn doesnt_work_when_passing_unrecognized_flag() -> Result<(), Box<dyn std::error::Error>> {
    let output_err = Command::cargo_bin("arg_parse")?
        .args(["--does-not-exist"])
        .output()
        .unwrap()
        .stderr;

    let expected = "error: unexpected argument '--does-not-exist' found\n\n  note: to pass '--does-not-exist' as a value, use '-- --does-not-exist'\n\nUsage: arg_parse [OPTIONS] <ARG1> [ARG2]\n\nFor more information, try '--help'.\n";

    assert_eq!(str::from_utf8(&output_err).unwrap(), expected);

    Ok(())
}
