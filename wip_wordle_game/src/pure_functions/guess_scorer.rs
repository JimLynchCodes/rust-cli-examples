
use indexmap::IndexMap;

use crate::data_elements::data::GuessState;

pub fn score_guess(
    guess: String,
    secret_word: String,
    letters_in_word: usize,
    mut prev_letters_guessed: IndexMap<String, GuessState>,
) -> (IndexMap<String, GuessState>, bool) {
    for i in 0..letters_in_word {
        let guess_char = guess.chars().nth(i.into()).unwrap();
        let secret_char = secret_word.chars().nth(i.into()).unwrap();

        println!("Comparing char {i} of guess {guess_char} to {secret_char}");

        if guess_char == secret_char {
            prev_letters_guessed
                .insert(
                    guess_char.to_string(),
                    GuessState::InWordFoundLocation(i as u8),
                )
                .unwrap();
        } else if secret_word.contains(guess_char) {
            prev_letters_guessed
                .insert(guess_char.to_string(), GuessState::InWordUnknownLocation)
                .unwrap();
        } else {
            prev_letters_guessed
                .insert(guess_char.to_string(), GuessState::GuessedNotInWord)
                .unwrap();
        }
    }

    (prev_letters_guessed, guess == secret_word)
}
