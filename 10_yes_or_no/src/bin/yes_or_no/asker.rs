use std::error::Error;

use inquire::{Confirm, InquireError};

pub fn ask_are_you_cool() -> Result<bool, Box<dyn Error>> {
    Ok(Confirm::new("Yes or no?")
        .with_default(false)
        .with_help_message("I'm an example of some help text!")
        .prompt()?)
}
