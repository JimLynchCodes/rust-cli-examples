// use lingua::Language::English;
// use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

use symspell::{AsciiStringStrategy, SymSpell};
use zspell::{DictBuilder, Dictionary};

// TODO - make this faster... 😅
// pub fn is_valid_english_word_sym_spell(word: &str, secret_word: &str) -> bool {
//     let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

//     symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
//     // symspell.load_bigram_dictionary(
//     //     "./data/frequency_bigramdictionary_en_243_342.txt",
//     //     0,
//     //     2,
//     //     " ",
//     // );

//     let suggestions = symspell.lookup(&word, symspell::Verbosity::Top, 2);

//     (suggestions.len() > 0 && suggestions[0].distance == 0) || word == secret_word
// }

pub fn is_valid_english_word(word: &str, secret_word: &str) -> bool {
    let aff_content =
        std::fs::read_to_string("data/english-dictionary.aff").expect("failed to load config file");
    let dic_content = std::fs::read_to_string("data/english-dictionary.dic")
        .expect("failed to load wordlist file");

    // Use the builder pattern to create our `Dictionary` object
    let dict: Dictionary = DictBuilder::new()
        .config_str(&aff_content)
        .dict_str(&dic_content)
        .build()
        .expect("failed to build dictionary!");

    dict.check(word) || word == secret_word
}
