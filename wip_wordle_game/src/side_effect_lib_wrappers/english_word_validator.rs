// use lingua::Language::English;
// use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

use symspell::{AsciiStringStrategy, SymSpell};

// TODO - make this faster... 😅
pub fn is_valid_english_word(word: &str, secret_word: &str) -> bool {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    // symspell.load_bigram_dictionary(
    //     "./data/frequency_bigramdictionary_en_243_342.txt",
    //     0,
    //     2,
    //     " ",
    // );

    let suggestions = symspell.lookup(&word, symspell::Verbosity::Top, 2);

    (suggestions.len() > 0 && suggestions[0].distance == 0) || word == secret_word
}
