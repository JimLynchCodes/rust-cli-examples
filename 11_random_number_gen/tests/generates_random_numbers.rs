use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
///  Notice that I'm not asserting much here... Could possibly parse the numbers
///  and assert nothing weird, but we're already doing that in the rnd gen tests!
fn prints_corrent_output() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("random_int_generator")?
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert!(output_str.contains("random"));

    Ok(())
}
