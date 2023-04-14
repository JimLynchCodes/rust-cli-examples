use std::error::Error;

use git2::Repository;

pub fn clone_repo(
    url: &str,
    location: &str,
    repo_name: &str,
) -> Result<Repository, Box<dyn Error>> {
    Ok(Repository::clone(
        url,
        &format!("{}{}", location, repo_name),
    )?)
}
