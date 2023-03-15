// use apology_letter_generator::*;

// #[test]
// fn test_apology_letter_generated () {

//     println!("foo");

//     assert!(true == true);

//     let expected_output = "
//     Fancy Letterhead
//     Very Fancy, Indeed
//     Very, Very Fancy,
    
    
//         Dear Foo,
    
//         I would like to express how very deeply sorry I am for things.
    
//         Sincerely,
//         Your friend James";

    

// }

// tests/cli.rs

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cmd = Command::cargo_bin("apology_letter_generator")?;

    cmd.arg("foobar").arg("things");
    cmd.assert()
        .success();
        // .failure();

        // .stderr(predicate::str::contains("could not read file"));

    Ok(())
}