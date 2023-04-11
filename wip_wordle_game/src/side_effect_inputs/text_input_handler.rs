use indexmap::IndexMap;
use inquire::{length, required, validator::Validation, Text};

use crate::{
    data_elements::data::GuessState,
    pure_functions::colorizer::{get_color_for_letter, get_keyboard_letters_color},
    side_effect_lib_wrappers::english_word_validator::is_valid_english_word,
    side_effect_outputs::console_printer::print_deciding_if_valid_english_word,
};

pub fn get_guess_from_user(
    letters_in_word: usize,
    secret_word: String,
    prev_letters_guessed: &IndexMap<String, GuessState>,
) -> String {
    let styled_letters_remaining = prev_letters_guessed
        .iter()
        .map(|(key, guessed_state)| get_keyboard_letters_color(key, guessed_state))
        .collect::<Vec<String>>()
        .join(", ");

    let _hardcodeded_letters_remaining = format!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        get_color_for_letter("a", &prev_letters_guessed),
        get_color_for_letter("b", &prev_letters_guessed),
        get_color_for_letter("c", &prev_letters_guessed),
        get_color_for_letter("d", &prev_letters_guessed),
        get_color_for_letter("e", &prev_letters_guessed),
        get_color_for_letter("f", &prev_letters_guessed),
        get_color_for_letter("g", &prev_letters_guessed),
        get_color_for_letter("h", &prev_letters_guessed),
        get_color_for_letter("i", &prev_letters_guessed),
        get_color_for_letter("j", &prev_letters_guessed),
        get_color_for_letter("k", &prev_letters_guessed),
        get_color_for_letter("l", &prev_letters_guessed),
        get_color_for_letter("m", &prev_letters_guessed),
        get_color_for_letter("n", &prev_letters_guessed),
        get_color_for_letter("o", &prev_letters_guessed),
        get_color_for_letter("p", &prev_letters_guessed),
        get_color_for_letter("q", &prev_letters_guessed),
        get_color_for_letter("r", &prev_letters_guessed),
        get_color_for_letter("s", &prev_letters_guessed),
        get_color_for_letter("t", &prev_letters_guessed),
        get_color_for_letter("u", &prev_letters_guessed),
        get_color_for_letter("v", &prev_letters_guessed),
        get_color_for_letter("w", &prev_letters_guessed),
        get_color_for_letter("x", &prev_letters_guessed),
        get_color_for_letter("y", &prev_letters_guessed),
        get_color_for_letter("z", &prev_letters_guessed),
    );

    Text::new("➠")
        .with_help_message(&styled_letters_remaining)
        // .with_help_message(&hardcodeded_letters_remaining)
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
        .prompt()
        .unwrap()
        .to_lowercase()

}
