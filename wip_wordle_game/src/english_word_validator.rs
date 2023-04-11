// use ispell::SpellLauncher;

// pub fn is_valid_english_word(word: String) -> bool {
//     let mut checker = SpellLauncher::new()
//                         // .ispell()
//                         .launch()
//                         .unwrap();

//     let errors = checker.check("Testing iff it works").unwrap();

//     true
// }

// extern crate ispell;
// use ispell::SpellLauncher;

// fn main() {

use std::error::Error;

use lingua::Language::English;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

use symspell::{AsciiStringStrategy, SymSpell, Verbosity};

use inquire::validator::{StringValidator, Validation};
///
/// let validator = |input: &str| match input.chars().find(|c| c.is_numeric()) {
///     Some(_) => Ok(Validation::Valid),
///     None => Ok(Validation::Invalid(
///         "Your password should contain at least 1 digit".into(),
///     )),
/// };

// pub fn is_valid_english_word(word: String) -> Result<Validation, Box<dyn std::error::Error + Send + Sync + 'static>> {
pub fn is_valid_english_word(word: String) -> Result<Validation, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // let languages = vec![English];
    // let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
    // let detected_language: Option<Language> = detector.detect_language_of("languages are awesome");

    // detector.compute_language_confidence_values(text)

    // assert_eq!(detected_language, Some(English));

    let my_string = "foobar".to_string();

    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    // symspell.load_bigram_dictionary(
    //     "./data/frequency_bigramdictionary_en_243_342.txt",
    //     0,
    //     2,
    //     " ",
    // );

    let suggestions = symspell.lookup(&word, Verbosity::Top, 2);
    println!("{:?}", suggestions);

    if suggestions.len() > 0 && suggestions[0].distance == 0 {
      return Ok(Validation::Valid);
    }

    Ok(Validation::Invalid("not a valid word".into()))

    // let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him";
    // let compound_suggestions = symspell.lookup_compound(sentence, 2);
    // println!("{:?}", compound_suggestions);

    // let sentence = "whereisthelove";
    // let segmented = symspell.word_segmentation(sentence, 2);
    // println!("{:?}", segmented);
}

// To check if a string is a valid English word in Rust, you will need to use a dictionary or list of English words. One way to do this is to use a crate (Rust's version of a library) called
// rust-english-words
// . Here's an example of how to use it:

// use rust_english_words::dictionary::Dictionary;

// fn main() {
//     let my_string = "hello";
//     let is_valid_word = Dictionary::check(&my_string.to_lowercase());
//     println!("Is {} a valid English word? {}", my_string, is_valid_word);
// }

// This code imports the
// Dictionary
//  struct from the
// rust-english-words
//  crate, creates a string called
// my_string
// , converts it to lowercase (since the dictionary is all lowercase), and then checks if it is a valid English word using the
// check
//  method of the
// Dictionary
//  struct. The result is then printed to the console.

// Note that you will need to add
// rust-english-words
//  to your dependencies in
// Cargo.toml
//  in order to use it. Here's an example of what that might look like:

// [dependencies]
// rust-english-words = "0.4.0"

//
// }
