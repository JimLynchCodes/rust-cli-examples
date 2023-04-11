mod console_printer;
use console_printer::{im_thinking_of_a_word, print_initial_message};

mod args_parser;
use args_parser::read_args;

mod random_word_generator;
use random_word_generator::generate_random_word;

mod prompt_controller;
use prompt_controller::{initial_letters_hashmap, prompt_for_guess};

mod english_word_validator;
use english_word_validator::is_valid_english_word;

// const letters_in_english_alphabet: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

fn main() {
    print_initial_message();

    let args = read_args();

    // println!("letters in word: {}, guesses:{}", args.letters_in_word, args.guesses);

    let secret_word = generate_random_word(args.letters_in_word);

    im_thinking_of_a_word(args.letters_in_word);

    // println!("secret word: {}", secret_word);

    // let is_word = is_valid_english_word("foo".to_string());

    // is_valid_english_word("farg".to_string());

    // println!("is_word: {:?}", is_word);

    // prompt_first_guess();

    // let letters_in_english_alphabet: [&str; 26] = [
    //     "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
    //     "s", "t", "u", "v", "w", "x", "y", "z",
    // ];
    

    prompt_for_guess(
        args.guesses,
        secret_word,
        args.letters_in_word.into(),
        initial_letters_hashmap(),
        1,
        Vec::new()
    );
}
