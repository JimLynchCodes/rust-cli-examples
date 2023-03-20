use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("apology_letter_generator_main")?;

    cmd.arg("foobar").arg("things");
    cmd.assert().success();

    let actual_file_contents =
        std::fs::read_to_string("./example_output.txt").expect("could not read file");

    let expected_output = "\nFancy Letterhead\nVery Fancy, Indeed\nVery, Very Fancy,\n\n\n    Dear foobar,\n\n    I would like to express how very deeply sorry I am for things.\n\n    Sincerely,\n    Your friend James";

    assert_eq!(actual_file_contents, expected_output);

    Ok(())
}
