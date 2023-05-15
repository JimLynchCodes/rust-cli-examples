use crate::data_elements::data::GuessState;
use termion::{color, style};

pub fn get_colored_keyboard_letters(text: &str, guessed_state: &GuessState) -> String {
    match guessed_state {
        GuessState::InWordFoundLocation(_) => {
            format!(
                "{}{}{}{}",
                color::Fg(color::Rgb(0, 1, 2)),
                color::Bg(color::Rgb(0, 1, 2)),
                text,
                style::Reset
            )

            // style(text.to_string()).magenta().on_green().to_string()
            // text.to_string()
            // text_style::StyledString::plain("test".to_string()).bold().style({fg})
            //     StyledString::styled(text.to_string(), text_style::Style {
            //     fg: Some(text_style::AnsiColor::Magenta.dark()),
            //     bg: Some(text_style::Color::Rgb { r: 20, g: 20, b: 20 }),
            //     effects: None as text_style::Style
            //     // style(text.to_string()).magenta().on_green().to_string()
            // }
        }
        GuessState::InWordUnknownLocation =>
        // text.to_string(),
        // {
        //     style(text.to_string()).magenta().on_green().to_string()
        // }
        {
            format!(
                "{}{}{}{}",
                color::Fg(color::Rgb(0, 1, 2)),
                color::Bg(color::Rgb(0, 1, 2)),
                text,
                style::Reset
            )
        }

        // style(text.to_string()).magenta().on_yellow().to_string()
        GuessState::GuessedNotInWord =>
        // style(text.to_string()).magenta().on_green().to_string(),
        {
            format!(
                "{}{}{}{}",
                color::Fg(color::Rgb(0, 1, 2)),
                color::Bg(color::Rgb(0, 1, 2)),
                text,
                style::Reset
            )
        }

        // text.to_string(),
        // style(text.to_string()).black().on_black().to_string(),
        GuessState::Unguessed =>
        // style(text.to_string()).magenta().on_green().to_string(),
        {
            format!(
                "{}{}{}{}",
                color::Fg(color::Rgb(0, 1, 2)),
                color::Bg(color::Rgb(0, 1, 2)),
                text,
                style::Reset
            )
        } // style(text.to_string()).black().on_white().to_string(),

          // text.to_string()
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
