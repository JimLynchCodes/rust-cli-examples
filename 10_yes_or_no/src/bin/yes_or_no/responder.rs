use inquire::InquireError;

pub fn get_response_from_answer(ans: bool) -> String {
    if ans {
        "You chose yes!".to_string()
    } else {
        "You chose no...".to_string()
    }
}

#[cfg(test)]
mod responder_tests {

    use inquire::InquireError;

    use crate::responder::get_response_from_answer;

    #[test]
    fn response_for_yes() {
        assert_eq!(get_response_from_answer(true), "You chose yes!");
    }

    #[test]
    fn response_for_no() {
        assert_eq!(get_response_from_answer(false), "You chose no...");
    }

    // #[test]
    // fn response_for_error() {
    //     assert_eq!(
    //         get_response_from_answer(Err(InquireError::Custom("foo".into()))),
    //         "Error with questionnaire, try again later"
    //     );
    // }
}
