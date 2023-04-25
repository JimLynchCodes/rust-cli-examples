use console::style;

use crate::data_elements::data::GuessState;

pub fn get_colored_keyboard_letters(text: &str, guessed_state: &GuessState) -> String {
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

// pub fn get_color_for_keyboard_letter(
//     letter: &str,
//     letters_guessed: &IndexMap<String, GuessState>,
// ) -> Result<StyledObject<String>, Box<dyn Error>> {
//     Ok(match letters_guessed.get(letter) {
//         Some(GuessState::InWordFoundLocation(_)) => style(letter.to_string()).magenta().on_green(),
//         Some(GuessState::InWordUnknownLocation) => style(letter.to_string()).magenta().on_yellow(),
//         Some(GuessState::GuessedNotInWord) => style(letter.to_string()).black().on_black(),
//         Some(GuessState::Unguessed) => style(letter.to_string()).black().on_white(),
//         None => panic!("couldn't get proper color..."),
//     })
// }
