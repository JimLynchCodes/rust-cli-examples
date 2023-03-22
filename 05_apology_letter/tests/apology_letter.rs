use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn generates_apology_letter() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("apology_letter")?;

    let arg1 = "foo";
    let arg2 = "things";

    cmd.arg(arg1).arg(arg2);
    cmd.assert().success();

    let actual_file_contents = std::fs::read_to_string("./example_output.txt").expect("could not read file");

    let expected_output = format!("\nFancy Letterhead\nVery Fancy, Indeed\nVery, Very Fancy\n\n\n    Dear {},\n\n    I would like to express how very deeply sorry I am for {}.\n\n    Sincerely,\n    Your friend James", arg1, arg2);

    assert_eq!(actual_file_contents, expected_output);

    Ok(())
}
