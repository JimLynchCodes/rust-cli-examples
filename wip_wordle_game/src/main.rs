mod data_elements {
    pub(super) mod data;
}
use data_elements::data::initial_letters_index_map;

mod pure_functions {
    pub(super) mod colorizer;
    pub(super) mod guess_scorer;
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

fn main() {
    let args = read_args();

    print_initial_message();

    let secret_word = generate_random_word(args.letters_in_word);

    print_im_thinking_of_a_word(args.letters_in_word);

    prompt_for_guess(
        args.guesses,
        secret_word,
        args.letters_in_word.into(),
        initial_letters_index_map(),
        1,
        Vec::new(),
        args.debug,
    );
}
