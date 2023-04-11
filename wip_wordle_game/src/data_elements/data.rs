use indexmap::IndexMap;

pub fn initial_letters_index_map() -> IndexMap<String, GuessState> {
    let letters_in_english_alphabet: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    IndexMap::from(letters_in_english_alphabet.map(|l: &str| (l.to_string(), GuessState::Unguessed)))
}

pub enum GuessState {
    Unguessed,
    GuessedNotInWord,
    InWordUnknownLocation,
    InWordFoundLocation(u8),
}