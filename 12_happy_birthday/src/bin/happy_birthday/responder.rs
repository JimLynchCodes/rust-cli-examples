use chrono::{Datelike, Local, NaiveDate};

pub fn respond_to_birthday(selected: NaiveDate) -> String {
    let today = Local::now().naive_local().date();

    let selected_ordinal_day: u32 = selected.ordinal();
    let current_ordinal_day: u32 = today.ordinal();

    let days_until_bday = if current_ordinal_day > selected_ordinal_day {
        365 - current_ordinal_day + selected_ordinal_day
    } else {
        selected_ordinal_day - current_ordinal_day
    };

    match days_until_bday {
        0 => "Happy Birthday!! 🥳 🎉 🎂 🎉 🥳".to_string(),
        1 => "Your birthday is tomorrow! Are you excited??".to_string(),
        _ => format!("Your birthday is in {} days!", days_until_bday),
    }
}

#[cfg(test)]
mod responder_tests {

    use chrono::{Duration, Local};

    use crate::responder::respond_to_birthday;

    #[test]
    fn responds_happy_birthday() {
        let today = Local::now().naive_local().date();
        let response = respond_to_birthday(today);
        assert_eq!(response, "Happy Birthday!! 🥳 🎉 🎂 🎉 🥳");
    }

    #[test]
    fn responds_birthday_is_tomorrow() {
        let today = Local::now().naive_local().date();
        let response = respond_to_birthday(today + Duration::days(1));
        assert_eq!(response, "Your birthday is tomorrow! Are you excited??");
    }

    #[test]
    fn responds_birthday_is_in_2_days() {
        let today = Local::now().naive_local().date();
        let response = respond_to_birthday(today + Duration::days(2));
        assert_eq!(response, "Your birthday is in 2 days!");
    }

    #[test]
    fn responds_birthday_is_in_364_days() {
        let today = Local::now().naive_local().date();
        let response = respond_to_birthday(today - Duration::days(1));
        assert_eq!(response, "Your birthday is in 364 days!");
    }
}
