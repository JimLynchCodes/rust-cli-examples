mod data_elements {
    pub(super) mod data;
}
use std::error::Error;

use data_elements::data::initial_letters_index_map;

mod pure_functions {
    pub(super) mod guess_colorizer;
    pub(super) mod guess_scorer;
    pub(super) mod keyboard_letters_colorizer;
}

mod side_effect_inputs {
    pub(super) mod args_parser;
    pub(super) mod text_input_handler;
}
use side_effect_inputs::args_parser::read_args;

mod side_effect_lib_wrappers {
    pub(super) mod english_word_validator;
    pub(super) mod random_word_generator;
}

use side_effect_lib_wrappers::random_word_generator::generate_random_word;

mod side_effect_outputs {
    pub(super) mod console_printer;
    pub(super) mod render_config;
}

use side_effect_outputs::console_printer::{print_im_thinking_of_a_word, print_initial_message};

mod game_manager;
use game_manager::prompt_for_guess;
use zspell::{DictBuilder, Dictionary};

fn main() -> Result<(), Box<dyn Error>> {
    let args = read_args();

    print_initial_message();

    let secret_word = generate_random_word(args.letters_in_word)?;

    print_im_thinking_of_a_word(args.letters_in_word);

    let aff_content =
        std::fs::read_to_string("data/english-dictionary.aff").expect("failed to load config file");
    let dic_content =
        std::fs::read_to_string("data/english-dictionary.dic").expect("failed to load wordlist file");

    // Use the builder pattern to create our `Dictionary` object
    let dict: Dictionary = DictBuilder::new()
        .config_str(&aff_content)
        .dict_str(&dic_content)
        .build()
        .expect("failed to build dictionary!");

    prompt_for_guess(
        args.guesses,
        secret_word,
        args.letters_in_word.into(),
        initial_letters_index_map(),
        1,
        Vec::new(),
        args.debug,
        dict
    );

    Ok(())
}
