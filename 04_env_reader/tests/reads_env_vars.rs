use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn reads_env_1_and_env_2() -> Result<(), Box<dyn std::error::Error>> {
    let mock_value = "mock value";

    std::env::set_var("MY_VAR_2", mock_value);

    let output_bytes = Command::cargo_bin("env_reader")?
        .envs([("MY_VAR_2", mock_value)])
        .output()?
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected =
        "value for the env var MY_VAR_1 is: hello!!\nvalue for the env var MY_VAR_2 is: mock val\n";

    assert_eq!(output_str, expected);

    Ok(())
}

#[test]
fn reads_env_1_uses_default_env_2() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("env_reader")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    let expected = "value for the env var MY_VAR_1 is: hello!!\nvalue for the env var MY_VAR_2 is: default val\n";

    assert_eq!(output_str, expected);

    Ok(())
}
