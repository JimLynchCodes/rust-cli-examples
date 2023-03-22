/**
    NOTE: Before running the code and tests you will need to manually add MY_VAR_3 to either the .env file or the local shell environment with something like `export MY_VAR_3=foo`
*/
use dotenv_codegen::dotenv;

fn main() {
    dotenv::dotenv().ok();
    println!(
        "value for the env var MY_VAR_1 is: {}",
        String::from(dotenv!("MY_VAR_1"))
    );
    println!(
        "value for the env var MY_VAR_2 is: {}",
        String::from(dotenv!("MY_VAR_2"))
    );
    println!(
        "value for the env var MY_VAR_3 is: {}",
        String::from(dotenv!("MY_VAR_3"))
    );
}
