use std::error::Error;

pub fn generate_random_word(letters: u8) -> Result<String, Box<dyn Error>> {
    Ok(random_word::gen_len(letters.into())?.to_string())
}
