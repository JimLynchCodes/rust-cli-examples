pub fn generate_random_word(letters: u8) -> String {
    random_word::gen_len(letters.into()).unwrap().to_string()
}
