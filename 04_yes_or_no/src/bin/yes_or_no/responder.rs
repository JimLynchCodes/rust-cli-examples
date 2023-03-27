use inquire::InquireError;

pub fn get_response_from_answer(ans: Result<bool, InquireError>) -> String {
    match ans {
        Ok(true) => "You chose yes!".to_string(),
        Ok(false) => "You chose no...".to_string(),
        Err(_) => "Error with questionnaire, try again later".to_string(),
    }
}

#[cfg(test)]
mod responder_tests {

    use inquire::InquireError;

    use crate::responder::get_response_from_answer;

    #[test]
    fn response_for_yes() {
        assert_eq!(
            get_response_from_answer(Ok(true)),
            "You chose yes!"
        );
    }

    #[test]
    fn response_for_no() {
        assert_eq!(
            get_response_from_answer(Ok(false)),
            "You chose no..."
        );
    }

    #[test]
    fn response_for_error() {
        assert_eq!(
            get_response_from_answer(Err(InquireError::Custom("foo".into()))),
            "Error with questionnaire, try again later"
        );
    }
}
