pub enum Highlights {
    GREEN,
    YELLOW,
    GRAY,
}

pub fn score_guess_old(guess: String, secret_word: String) -> [Highlights] {
    return guess.chars().enumerate().map(|(index, char)| {
        if char == secret_word.char_at(index) {
            Highlights::GREEN
        } else if secret_word.contains(char) {
            Highlights::YELLOW
        } else {
            Highlights::GRAY
        }
    });
}


// pub fn score_guess()
