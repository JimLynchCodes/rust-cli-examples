/**
 *  An "Integration Test" that calls this program, simulating
 *
 *  a user running the binary from the command line.
 *
 *  Note that the libraries "assert_cmd" and "predicates" were added with
 *
 *  the command: `cargo add assert_cmd predicates`
 *
 *  NOTE: these tests don't really seem to work...
 *
 */
use assert_cmd::prelude::*; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use std::env;

#[test]
#[should_panic]
fn panics_when_expected_env_var_is_not_defined() -> Result<(), Box<dyn std::error::Error>> {
    
    // let mut cmd = 
    
    Command::cargo_bin("env_reader")

    cmd.assert().failure().unwrap()

    Ok(());

}

#[test]
fn works_when_all_env_vars_are_defined() -> Result<(), Box<dyn std::error::Error>> {
    
    env::set_var("MY_VAR_3", "something");
    
    let key = "MY_VAR_3";
    env::set_var(key, "VALUE");
    assert_eq!(env::var(key), Ok("VALUE".to_string()));

    // let mut cmd = Command::cargo_bin("env_reader")?;
    // cmd.assert().success();

    Ok(())
}

