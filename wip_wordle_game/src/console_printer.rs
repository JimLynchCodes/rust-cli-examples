use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use console::{style, StyledObject};

use crate::prompt_controller::GuessState;

pub fn print_initial_message() {
    println!("\nLet's play a word guessing game!");
    sleep(Duration::from_secs(1));
}

pub fn im_thinking_of_a_word(letters: u8) {
    println!("\nI'm thinking of a random {letters} letter english word...");
    sleep(Duration::from_secs(1));
}

pub fn you_have_x_guesses_left(remaining_guesses: u8) {
    println!("\nYou have {remaining_guesses} guesses remaining!\n");
    sleep(Duration::from_secs(1));
}

pub fn print_welcome_message() {}

pub fn print_scored_guess(word: &str, new_letters: &HashMap<String, GuessState>) -> String {
    println!("scoring...");

    let scored_guess = word.chars().into_iter().map({
        |char| get_score_style_for_guess_state(char, new_letters.get(&char.to_string()).unwrap())
    })
    // .collect::Vec<StyledObject<String>().unwrap();
    .collect::<Vec<String>>()
    .join("");



    // let formatted_scored_guess = format!("{:?}", scored_guess);
    // let formatted_scored_guess = format!("{:?}", scored_guess);

   scored_guess
}

pub fn print_you_win(guesses: u8) {
    println!("you won in {guesses} guesses!");
}

pub fn print_you_lose() {
    println!("You ran out of guesses! Better luck next time.");
}

fn get_score_style_for_guess_state(text: char, guess_state: &GuessState) -> String {
    match guess_state {
        GuessState::Guessed_Not_In_Word => style(text.to_string()).white().on_black().to_string(),
        GuessState::In_Word_Found_Location(_) => style(text.to_string()).white().on_green().to_string(),
        GuessState::In_Word_Unknown_Location => style(text.to_string()).white().on_yellow().to_string(),
        GuessState::Unguessed => panic!("shouldn't be able to get here..."),
    }
}
