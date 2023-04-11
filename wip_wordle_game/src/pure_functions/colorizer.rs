use console::{style, StyledObject};
use indexmap::IndexMap;

use crate::data_elements::data::GuessState;

pub fn get_score_style_for_guess_state(text: char, guess_state: &GuessState) -> String {
    match guess_state {
        GuessState::GuessedNotInWord => style(text.to_string()).white().on_black().to_string(),
        GuessState::InWordFoundLocation(_) => {
            style(text.to_string()).white().on_green().to_string()
        }
        GuessState::InWordUnknownLocation => {
            style(text.to_string()).white().on_yellow().to_string()
        }
        GuessState::Unguessed => panic!("shouldn't be able to get here..."),
    }
}

pub fn scored_guess_string(word: &str, new_letters: &IndexMap<String, GuessState>) -> String {
    println!("\nScoring guess...\n");

    word.chars()
        .into_iter()
        .map({
            |char| {
                get_score_style_for_guess_state(char, new_letters.get(&char.to_string()).unwrap())
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn get_keyboard_letters_color(text: &str, guessed_state: &GuessState) -> String {
    match guessed_state {
        GuessState::InWordFoundLocation(_) => {
            style(text.to_string()).magenta().on_green().to_string()
        }
        GuessState::InWordUnknownLocation => {
            style(text.to_string()).magenta().on_yellow().to_string()
        }
        GuessState::GuessedNotInWord => style(text.to_string()).black().on_black().to_string(),
        GuessState::Unguessed => style(text.to_string()).black().on_white().to_string(),
    }
}

pub fn get_color_for_letter(
    letter: &str,
    letters_guessed: &IndexMap<String, GuessState>,
) -> StyledObject<String> {
    match letters_guessed.get(letter).unwrap() {
        GuessState::InWordFoundLocation(_) => style(letter.to_string()).magenta().on_green(),
        GuessState::InWordUnknownLocation => style(letter.to_string()).magenta().on_yellow(),
        GuessState::GuessedNotInWord => style(letter.to_string()).black().on_black(),
        GuessState::Unguessed => style(letter.to_string()).black().on_white(),
    }
}
