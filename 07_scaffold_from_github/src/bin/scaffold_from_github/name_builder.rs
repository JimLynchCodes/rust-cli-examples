
pub fn get_repo_name(url: &str) -> String {
    
   url.split("/").collect::<Vec<&str>>().pop().unwrap().replace(".git", "")

}

#[test]
fn gets_repo_name() {

    use crate::name_builder::get_repo_name;
    
    let example_url = "https://github.com/vivainio/rraf";
    let expected = "rraf";

    assert_eq!(expected, &get_repo_name(example_url));

}
