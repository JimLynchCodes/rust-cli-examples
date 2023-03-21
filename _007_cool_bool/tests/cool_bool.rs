use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

// #[test]
// fn works_when() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("cool_bool")?;

//     // cmd;
//     cmd.assert().success();

//     Ok(())
// }

#[test]
fn works() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mybin")?;

    cmd.arg("foobar").arg("things");
    cmd.assert().success();

    Ok(())
}

// #[test]
// fn doesnt_work_when_passed_zero_or_one_args() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("basic_arg_parse")?;

//     cmd.assert().failure();

//     cmd.arg("foobar");
//     cmd.assert().failure();

//     Ok(())
// }
