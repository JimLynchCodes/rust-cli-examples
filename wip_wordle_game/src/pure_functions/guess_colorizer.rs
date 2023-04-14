use std::error::Error;

use console::style;
use indexmap::IndexMap;

use crate::data_elements::data::GuessState;

fn get_score_style_for_guess_state(text: char, guess_state: &GuessState) -> String {
    match guess_state {
        GuessState::GuessedNotInWord => style(text.to_string()).white().on_black().to_string(),
        GuessState::InWordFoundLocation(_) => {
            style(text.to_string()).black().on_green().to_string()
        }
        GuessState::InWordUnknownLocation => {
            style(text.to_string()).black().on_yellow().to_string()
        }
        GuessState::Unguessed => panic!("shouldn't be able to get here..."),
    }
}

pub fn build_colored_guess_string(
    word: &str,
    new_letters: &IndexMap<String, GuessState>,
) -> Result<String, Box<dyn Error>> {
    println!("\nScoring guess...\n");

    Ok(word
        .chars()
        .into_iter()
        .map({ |char| get_score_style_for_guess_state(char, new_letters.get(&char.to_string())?) })
        .collect::<Vec<String>>()
        .join(""))
}

#[cfg(test)]
mod build_colored_guess_tests {
    use indexmap::IndexMap;

    use crate::data_elements::data::GuessState;

    use super::build_colored_guess_string;

    #[test]
    fn builds_colored_guess() -> Result<String, Box<dyn Error>> {
        let mock_word = "abc";

        let letters_map = IndexMap::from([
            ("a".to_string(), GuessState::GuessedNotInWord),
            ("b".to_string(), GuessState::InWordFoundLocation(1)),
            ("c".to_string(), GuessState::InWordUnknownLocation),
            // ("d".to_string(), GuessState::Unguessed),
        ]);

        let actual = build_colored_guess_string(mock_word, &letters_map)?;

        let expected = "\u{1b}[37m\u{1b}[40ma\u{1b}[0m\u{1b}[30m\u{1b}[42mb\u{1b}[0m\u{1b}[30m\u{1b}[43mc\u{1b}[0m";

        assert_eq!(actual, expected);
    }
}
