use std::error::Error;

use chrono::{Local, NaiveDate};
use inquire::DateSelect;

pub fn ask_for_birthday() -> Result<NaiveDate, Box<dyn Error>> {
    let today = Local::now().naive_local().date();
    let starting_date = NaiveDate::from_ymd_opt(1990, 1, 1).ok_or(format!(
        "couldn't parse starting date from \"today\": {today}"
    ))?;
    let earliest_possible_date =
        NaiveDate::from_ymd_opt(1800, 1, 1).ok_or(format!("couldn't parse earliest date..."))?;

    Ok(DateSelect::new("On what day were you born?")
        .with_starting_date(starting_date)
        .with_min_date(earliest_possible_date)
        .with_max_date(today)
        .with_help_message("Use the arrow keys to move cursor and space or enter to select a date")
        .prompt()?)
}
