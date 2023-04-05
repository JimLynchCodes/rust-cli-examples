use std::fs;

pub fn delete_git_folder(location: String) {
    fs::remove_dir_all(&format!("{}/.git", location)).unwrap();
}