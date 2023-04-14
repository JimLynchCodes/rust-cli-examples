use indexmap::IndexMap;

#[derive(Debug, PartialEq)]
pub enum GuessState {
    Unguessed,
    GuessedNotInWord,
    InWordUnknownLocation,
    InWordFoundLocation(u8),
}

pub fn initial_letters_index_map() -> IndexMap<String, GuessState> {
    // TODO - why doesn't soemthing like this work?
    // let letters_in_english_alphabet2 = "a".."z".iter().collect::Vec<&str>();

    let letters_in_english_alphabet: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    IndexMap::from(
        letters_in_english_alphabet.map(|l: &str| (l.to_string(), GuessState::Unguessed)),
    )
}

#[cfg(test)]
mod initial_letters_index_map_tests {

    use std::error::Error;

    use crate::data_elements::data::GuessState;

    use super::initial_letters_index_map;

    #[test]
    fn returns_initial_letters_index_map() -> Result<(), Box<dyn Error>> {
        let expected_keys_and_values = [
            ("a", GuessState::Unguessed),
            ("b", GuessState::Unguessed),
            ("c", GuessState::Unguessed),
            ("d", GuessState::Unguessed),
            ("e", GuessState::Unguessed),
            ("f", GuessState::Unguessed),
            ("g", GuessState::Unguessed),
            ("h", GuessState::Unguessed),
            ("i", GuessState::Unguessed),
            ("j", GuessState::Unguessed),
            ("k", GuessState::Unguessed),
            ("l", GuessState::Unguessed),
            ("m", GuessState::Unguessed),
            ("n", GuessState::Unguessed),
            ("o", GuessState::Unguessed),
            ("p", GuessState::Unguessed),
            ("q", GuessState::Unguessed),
            ("r", GuessState::Unguessed),
            ("s", GuessState::Unguessed),
            ("t", GuessState::Unguessed),
            ("u", GuessState::Unguessed),
            ("v", GuessState::Unguessed),
            ("w", GuessState::Unguessed),
            ("x", GuessState::Unguessed),
            ("y", GuessState::Unguessed),
            ("z", GuessState::Unguessed),
        ];

        let actual_index_map = initial_letters_index_map();

        for (key, value) in expected_keys_and_values {
            let actual_val = actual_index_map.get(key)?;

            assert_eq!(actual_val, &value);
        }

        Ok(())
        //
        // assert_eq!(initial_letters_index_map(), expected);
    }
}
