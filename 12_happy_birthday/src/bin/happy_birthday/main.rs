mod asker;
use std::error::Error;

use crate::asker::ask_for_birthday;

mod responder;
use crate::responder::respond_to_birthday;

fn main() -> Result<(), Box<dyn Error>> {
    let date_of_birth = ask_for_birthday()?;

    let response = respond_to_birthday(date_of_birth);

    println!("{}", response);

    Ok(())
}
