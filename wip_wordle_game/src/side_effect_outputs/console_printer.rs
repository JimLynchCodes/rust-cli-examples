use std::thread::sleep;
use std::time::Duration;

pub fn print_initial_message() {
    println!("\nLet's play a word guessing game!");
    sleep_for_cool_ux();
}

pub fn print_im_thinking_of_a_word(letters: u8) {
    println!("\nI'm thinking of a random {letters} letter english word...");
    sleep_for_cool_ux();
}

pub fn print_guesses_left(remaining_guesses: u8) {
    println!("\nYou have {remaining_guesses} guesses remaining!\n");
    sleep_for_cool_ux();
}

pub fn print_prev_scored_guesses(scored_guesses: &Vec<String>) {
    println!("Your Guesses:");
    for prev_scored_guess in scored_guesses {
        println!("{prev_scored_guess}");
    }
    sleep_for_cool_ux();
}

pub fn print_you_win(guesses: u8, secret_word: &str) {
    println!("\nYou got it!! The word was \"{}\"", secret_word);
    println!("\nYou won in {guesses} guesses!\n");
    sleep_for_cool_ux();
}

pub fn print_you_lose(scored_guesses: &Vec<String>, secret_word: &str) {
    println!("\nYou ran out of guesses! Better luck next time.\n");
    sleep_for_cool_ux();

    print_prev_scored_guesses(scored_guesses);

    println!("\nThe word I was thinking of was: {}\n", secret_word);
    sleep_for_cool_ux();
}

pub fn print_deciding_if_valid_english_word(word: &str) {
    println!(
        "\n\n{}",
        format!("Deciding if {word} is a valid english word...")
    );
    sleep_for_cool_ux();
}

const MILLIS_TO_SLEEP: u64 = 650;

fn sleep_for_cool_ux() {
    sleep(Duration::from_millis(MILLIS_TO_SLEEP));
}
