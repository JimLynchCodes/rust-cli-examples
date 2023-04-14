use std::error::Error;

pub fn get_repo_name(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(url
        .split("/")
        .collect::<Vec<&str>>()
        .pop()
        .ok_or(format!("couln't parse repo name from url: {url}"))?
        .to_string())
}

#[test]
fn gets_repo_name() -> Result<(), Box<dyn Error>> {
    use crate::name_builder::get_repo_name;

    let example_url = "https://github.com/vivainio/rraf";
    let expected = "rraf";

    assert_eq!(expected, &get_repo_name(example_url)?);

    Ok(())
}
