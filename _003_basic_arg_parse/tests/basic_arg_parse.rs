/**
 *  An "Integration Test" that calls this program, simulating
 *
 *  a user running the binary from the command line.
 *
 *  Note that the libraries "assert_cmd" and "predicates" were added with
 *
 *  the command: `cargo add assert_cmd predicates`
 *
 */
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn works_when_passed_two_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("basic_arg_parse")?;

    cmd.arg("foobar").arg("things");
    cmd.assert().success();

    Ok(())
}

#[test]
fn doesnt_work_when_passed_zero_or_one_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("basic_arg_parse")?;

    cmd.assert().failure();

    cmd.arg("foobar");
    cmd.assert().failure();

    Ok(())
}
