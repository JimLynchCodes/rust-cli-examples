use std::str;
use std::process::Command;
use assert_cmd::prelude::*;

#[test]
#[ignore]
fn asks_are_you_cool() -> Result<(), Box<dyn std::error::Error>> {

    let output_bytes = Command::cargo_bin("cool_bool")?.output().unwrap().stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert_eq!(output_str, "That's too bad, I though you might have been.\n");

    Ok(())
}

#[test]
fn displays_error_message_on_error() -> Result<(), Box<dyn std::error::Error>> {

    let output_bytes = Command::cargo_bin("cool_bool")?.output().unwrap().stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert_eq!(output_str, "Error with questionnaire, try again later\n");

    Ok(())
}
