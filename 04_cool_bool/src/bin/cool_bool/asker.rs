use inquire::{Confirm, InquireError};

pub fn ask_are_you_cool() -> Result<bool, InquireError> {
    Confirm::new("Are you cool?")
        .with_default(false)
        .with_help_message("(It's not a trick question)")
        .prompt()
}