// #[macro_use]
// extern crate dotenv_codegen::dotenv;
use dotenv_codegen::dotenv;
// use dotenv;

fn main() {
    dotenv::dotenv().ok();
    println!("value for the env var MY_VAR_1 is: {}", String::from(dotenv!("MY_VAR_1")));
    println!("value for the env var MY_VAR_2 is: {}", String::from(dotenv!("MY_VAR_2")));
    println!("value for the env var MY_VAR_3 is: {}", String::from(dotenv!("MY_VAR_3")));
}
