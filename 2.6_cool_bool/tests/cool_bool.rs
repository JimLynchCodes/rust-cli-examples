use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn runs_successfully() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cool_bool")?;

    cmd.assert().success();

    Ok(())
}
