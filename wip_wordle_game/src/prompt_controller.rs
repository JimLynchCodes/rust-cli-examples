// pub fn prompt_first_guess(guesses) {

// }

pub fn prompt_for_guess(remaining_guesses: u8, secret_word: String, unguessed_letters: [&str; 26]) {

    // let guess = inquire :: Text

    // unwrap / validate guess (check if valid word)

    // let score = score_guess(guess, secret) (decide what to color green and yellow)

    // if (perfect_match) {
    // console_printer::print_you_win(remaining_guesses)
    // }
    // else if (guesses == 1) {
    //     console_printer::print_you_lose()
    // }
    // else {
    //     console_printer::print_scored_guess();
    //
    //     let new_unguessed_letters = calculate_letters_remaining(unguessed_letters, guess);
    //
    //     prompt_next_guess(remaining_guesses - 1, secret_word, new_unguessed_letters);
    // }
    //
    //
    // match score
    //   perfect match => console_printer::print_you_win(remaining_guesses),
    //   remaining guesses == 1 => console_printer::print_you_lose()
    //   _ => {
    //        console_printer::print_scored_guess();
    //
    //        let new_unguessed_letters = calculate_letters_remaining(unguessed_letters, guess);
    //
    //        prompt_next_guess(remaining_guesses - 1, secret_word, new_unguessed_letters);
    //    };
}

fn win() {}

fn lose() {}
