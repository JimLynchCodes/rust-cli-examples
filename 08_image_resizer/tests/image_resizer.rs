use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[ignore] // Note: this test takes a while because it's actually generating the images!
fn generates_resized_images() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("image_resizer")?;

    cmd.assert().success();
    
    let expected_file_names = vec![
        "spacy_jim_1-2x.png",
        "spacy_jim_1-4x.png",
        "spacy_jim_1-8x.png",
        "spacy_jim_2x.png",
        "spacy_jim_4x.png",
        "spacy_jim_8x.png",
        ];
        
    for expected in expected_file_names {
        let actual_file_contents = image::open(format!("./output_images/{}", expected)).expect("could not read image file");
        let expected_file_contents = image::open(format!("./expected_output_images/{}", expected)).expect("could not read image file");

        assert_eq!(actual_file_contents, expected_file_contents);
    }

    Ok(())
}
