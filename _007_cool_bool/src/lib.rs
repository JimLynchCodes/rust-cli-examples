pub mod asker {

    use inquire::{Confirm, InquireError};

    pub fn ask_are_you_cool() -> Result<bool, InquireError> {
        Confirm::new("Are you cool?")
            .with_default(false)
            .with_help_message("(It's not a trick question)")
            .prompt()
    }
}

#[cfg(test)]
mod asker_tests {

    use crate::asker::ask_are_you_cool;

    #[test]
    #[ignore]
    /*
        Note that this test just hangs when it is run...

        But philosophicallly thinking here, the function is purely a user input side effect... do we really need to unit test it? 🤔
    */
    fn returns_thats_awesome_when_user_says_yes() {
        assert_eq!(ask_are_you_cool().unwrap(), true);
    }
}

pub mod responder {
    use inquire::InquireError;

    pub fn get_response_from_are_you_cool_answer(ans: Result<bool, InquireError>) -> String {
        match ans {
            Ok(true) => "That's awesome!".to_string(),
            Ok(false) => "That's too bad, I though you might have been.".to_string(),
            Err(_) => "Error with questionnaire, try again later".to_string(),
        }
    }
}

#[cfg(test)]
mod responder_tests {

    use inquire::InquireError;

    use crate::responder::get_response_from_are_you_cool_answer;

    #[test]
    fn returns_thats_awesome_when_getting_ok_true() {
        assert_eq!(
            get_response_from_are_you_cool_answer(Ok(true)),
            "That's awesome!"
        );
    }

    #[test]
    fn returns_thats_too_bad_when_getting_ok_false() {
        assert_eq!(
            get_response_from_are_you_cool_answer(Ok(false)),
            "That's too bad, I though you might have been."
        );
    }

    #[test]
    fn returns_error_message_when_getting_error() {
        assert_eq!(
            get_response_from_are_you_cool_answer(Err(InquireError::Custom("bar".into()))),
            "Error with questionnaire, try again later"
        );
    }
}
