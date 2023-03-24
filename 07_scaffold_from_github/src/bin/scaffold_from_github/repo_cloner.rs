use git2::Repository;

pub fn clone_repo(url: &str, location: &str, repo_name: &str) {
    
    Repository::clone(url, &format!("{}{}", location, repo_name)).expect("Couldn't scaffold repo... doens't it already exist here?");

}