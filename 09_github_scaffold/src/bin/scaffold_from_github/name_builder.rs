use std::error::Error;

pub fn get_repo_name(url: &str) -> String {
    url.split("/")
        .last()
        .expect(&format!("url \"{}\" contains no repo name...", url))
        .to_string()
}

#[test]
fn gets_repo_name() -> Result<(), Box<dyn Error>> {
    use crate::name_builder::get_repo_name;

    let example_url = "https://github.com/vivainio/rraf";
    let expected = "rraf";

    assert_eq!(expected, &get_repo_name(example_url));

    Ok(())
}
