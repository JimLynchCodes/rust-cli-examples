use std::{error::Error, fs};

pub fn delete_git_folder(location: String) -> Result<(), Box<dyn Error>> {
    Ok(fs::remove_dir_all(&format!("{}/.git", location))?)
}
