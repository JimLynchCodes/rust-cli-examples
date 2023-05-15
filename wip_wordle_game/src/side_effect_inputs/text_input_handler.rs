use std::error::Error;

use indexmap::IndexMap;
use inquire::{length, required, validator::Validation, Text};

use crate::{
    data_elements::data::GuessState,
    pure_functions::keyboard_letters_colorizer::get_colored_keyboard_letters,
    side_effect_lib_wrappers::english_word_validator::is_valid_english_word,
    side_effect_outputs::console_printer::print_deciding_if_valid_english_word,
};

pub fn get_guess_from_user(
    letters_in_word: usize,
    secret_word: String,
    prev_letters_guessed: &IndexMap<String, GuessState>,
) -> Result<String, Box<dyn Error>> {
    
    let styled_letters_remaining = prev_letters_guessed
        .iter()
        .map(|(key, guessed_state)| get_colored_keyboard_letters(key, guessed_state))
        .collect::<Vec<String>>()
        .join(" ");

    Ok(Text::new("➠")
        .with_help_message(&styled_letters_remaining)
        .with_validator(required!())
        .with_validator(length!(letters_in_word))
        .with_validator(move |word: &str| {
            print_deciding_if_valid_english_word(word);

            if is_valid_english_word(&word.to_lowercase(), &secret_word) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    format!("\n{} is not a valid english word.", word).into(),
                ))
            }
        })
        .prompt()?
        .to_lowercase())
}
