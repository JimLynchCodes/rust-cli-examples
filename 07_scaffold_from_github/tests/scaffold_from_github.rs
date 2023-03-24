use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;

fn delete_folder_if_it_exists(folder: &str) {
    if let Ok(metadata) = fs::metadata(format!("./cloned_repos/Integration-Test-Doc/README.md")) {
        if metadata.is_file() {
            match fs::remove_dir_all(folder) {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    }
}

#[test]
fn scaffolds_repo() -> Result<(), Box<dyn std::error::Error>> {
    delete_folder_if_it_exists("./cloned_repos/Integration-Test-Doc");

    let mut cmd = Command::cargo_bin("scaffold_from_github")?;
    cmd.assert().success();

    let actual_file_contents =
        std::fs::read_to_string("./cloned_repos/Integration-Test-Doc/README.md")
            .expect("could not read the file");

    assert_eq!(actual_file_contents, "# Integration-Test-Doc");

    Ok(())
}

#[test]
fn deletes_git_folder_after_scaffolding_repo() -> Result<(), std::io::Error> {
    delete_folder_if_it_exists("./cloned_repos/Integration-Test-Doc");

    Command::cargo_bin("scaffold_from_github").unwrap();

    let file_path = "./cloned_repos/Integration-Test-Doc/.git/HEAD";
    let result = fs::read_to_string(file_path);

    match result {
        Ok(_) => panic!("Found a .git folder when we shouldn't have..."),
        Err(m) => assert!(m.to_string().contains("No such file or directory")),
    };

    Ok(())
}
