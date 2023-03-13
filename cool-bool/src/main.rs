
use inquire::Confirm;

fn main() {
    let ans = Confirm::new("Are you cool?")
        .with_default(false)
        .with_help_message("(It's not a trick question)")
        .prompt();

    match ans {
        Ok(true) => println!("That's awesome!"),
        Ok(false) => println!("That's too bad, I though you might have been."),
        Err(_) => println!("Error with questionnaire, try again later"),
    }
}
