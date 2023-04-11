use std::collections::IndexMap;

use console::{style, StyledObject};
use inquire::validator::Validation;

use crate::pure_functions::colorizer::scored_guess_string;
use crate::side_effect_outputs::console_printer::{
    print_deciding_if_valid_english_word, print_you_lose, print_you_win, print_guesses_left,
};

use crate::side_effect_lib_wrappers::english_word_validator::is_valid_english_word;

use inquire::ui::{RenderConfig, Styled};

use inquire::{length, required, Text};

// let static mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
// let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();






