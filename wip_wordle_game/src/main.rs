
mod args_parser;
use args_parser::read_args;

mod random_word_generator;
use random_word_generator::generate_random_word;

mod prompt_controller;
use prompt_controller::prompt_for_guess;

// const letters_in_english_alphabet: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

fn main() {
    
    let args = read_args();
    
    let letters_in_english_alphabet: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    

    // println!("letters in word: {}, guesses:{}", args.letters_in_word, args.guesses);
    
    let secret_word = generate_random_word(args.letters_in_word);

    // println!("secret word: {}", secret_word);

    // prompt_first_guess();

    prompt_for_guess(args.guesses, secret_word, letters_in_english_alphabet);

}
