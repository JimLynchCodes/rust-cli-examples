use console::{style, StyledObject};
use indexmap::IndexMap;

use crate::data_elements::data::GuessState;

fn get_score_style_for_guess_state(text: char, guess_state: &GuessState) -> String {
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

pub fn build_colored_guess_string(word: &str, new_letters: &IndexMap<String, GuessState>) -> String {
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
