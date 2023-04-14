mod repo_cloner;
mod name_builder;
mod git_folder_deleter;

use std::error::Error;

use repo_cloner::clone_repo;
use name_builder::get_repo_name;
use git_folder_deleter::delete_git_folder;

extern crate git2;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://github.com/JimLynchCodes/Integration-Test-Doc";

    let where_to_clone = "./cloned_repos/";
    
    let repo_name = get_repo_name(url)?;

    clone_repo(&url, &where_to_clone, &repo_name)?;

    delete_git_folder(format!("./cloned_repos/{}", repo_name))?;

    Ok(())

}