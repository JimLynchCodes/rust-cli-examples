use std::error::Error;

use inquire::Confirm;

pub fn ask_yes_or_no() -> Result<bool, Box<dyn Error>> {
    Ok(Confirm::new("Yes or no?")
        .with_default(false)
        .with_help_message("I'm an example of some help text!")
        .prompt()?)
}

#[cfg(test)]
mod ask_yes_or_no_test {

    use std::error::Error;

    use inquire::Confirm;

    use crate::asker::ask_yes_or_no;

    
    #[test]
    fn creates_inquire_confirm() -> Result<(), Box<dyn Error>> {
        let expected_confirm_object = Confirm::new("Yes or no?")
        .with_default(false)
        .with_help_message("I'm an example of some help text!")
        .prompt()?;

        assert_eq!(ask_yes_or_no()?, expected_confirm_object);

        Ok(())
    }

}
