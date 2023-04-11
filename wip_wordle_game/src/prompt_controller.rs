// pub fn prompt_first_guess(guesses) {

// }

use std::collections::HashMap;
use std::default;
use std::hash::Hash;

use console::{style, Color, StyledObject};
use inquire::validator::Validation;
use symspell::{AsciiStringStrategy, SymSpell};
// use termion::color::AnsiValue;

use crate::console_printer::{print_scored_guess, you_have_x_guesses_left};

use crate::english_word_validator::is_valid_english_word;

use crate::console_printer::{print_you_lose, print_you_win};

use inquire::ui::{Attributes, RenderConfig, StyleSheet, Styled};

use inquire::{length, required, Text};

// let static mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

// let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

pub fn initialize_dictionary() {}

pub fn initial_letters_hashmap() -> HashMap<String, GuessState> {
    let letters_in_english_alphabet: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    HashMap::from(letters_in_english_alphabet.map(|l: &str| (l.to_string(), GuessState::Unguessed)))

    // HashMap::from([
    //     ("a".to_string(), GuessState::Unguessed),
    //     ("b".to_string(), GuessState::Unguessed),
    //     ("c".to_string(), GuessState::Unguessed),
    //     ("d".to_string(), GuessState::Unguessed),
    //     ("e".to_string(), GuessState::Unguessed),
    // ])
}

pub struct Letter {
    text: String,
    guess_state: GuessState,
}

pub enum GuessState {
    Unguessed,
    Guessed_Not_In_Word,
    In_Word_Unknown_Location,
    In_Word_Found_Location(u8),
}
// let name = Text::new(&format!("{} {}",
//     style("What ").cyan().on_blue(),
//     style("is your name?").red().on_yellow()))

// Letter {
// text: letter.to_string(),
// guess_state: GuessState::Unguessed

// let letters_str = format!("{:?}", styled_letters_remaining);
// let letters_str = styled_letters_remaining.join("");
// let letters_str = format!(" {} ", styled_letters_remaining.join(" "));
//
// let letters_str2 = format!(" {:?} ", unguessed_letters);

// let lots = unguessed_letters.map(|letter| style(letter).red().on_magenta().to_string());

// let letters_str2 = format!("{}",lots.join(""));
//
// let letters_str22 = format!(" {} ", style(unguessed_letters[0]).red().on_magenta().to_string());

pub fn prompt_for_guess(
    remaining_guesses: u8,
    secret_word: String,
    letters_in_word: usize,
    prev_letters_guessed: HashMap<String, GuessState>,
    guesses_already_made: u8,
    mut scored_guesses: Vec<String>,
) {
    inquire::set_global_render_config(get_render_config());

    you_have_x_guesses_left(remaining_guesses);

    let secret = secret_word.clone();

    let styled_letters_remaining = prev_letters_guessed
        .iter()
        .map(|(key, guessed_state)| get_color_for_guessed_state(key, guessed_state))
        // .map(|letter | Letter {
        //     text: letter.to_string(),
        //     guess_state: GuessState::Unguessed
        // });
        // .map(|letter | style(letter).fg(Color::Color256(3)).bg(Color::Color256(250)).to_string());
        .map(|letter| style(letter).magenta().on_black().to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let hardcodeded_letters_remaining = format!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        get_color_for_letter("a", &prev_letters_guessed),
        get_color_for_letter("b", &prev_letters_guessed),
        get_color_for_letter("c", &prev_letters_guessed),
        get_color_for_letter("d", &prev_letters_guessed),
        get_color_for_letter("e", &prev_letters_guessed),
        get_color_for_letter("f", &prev_letters_guessed),
        get_color_for_letter("g", &prev_letters_guessed),
        get_color_for_letter("h", &prev_letters_guessed),
        get_color_for_letter("i", &prev_letters_guessed),
        get_color_for_letter("j", &prev_letters_guessed),
        get_color_for_letter("k", &prev_letters_guessed),
        get_color_for_letter("l", &prev_letters_guessed),
        get_color_for_letter("m", &prev_letters_guessed),
        get_color_for_letter("n", &prev_letters_guessed),
        get_color_for_letter("o", &prev_letters_guessed),
        get_color_for_letter("p", &prev_letters_guessed),
        get_color_for_letter("q", &prev_letters_guessed),
        get_color_for_letter("r", &prev_letters_guessed),
        get_color_for_letter("s", &prev_letters_guessed),
        get_color_for_letter("t", &prev_letters_guessed),
        get_color_for_letter("u", &prev_letters_guessed),
        get_color_for_letter("v", &prev_letters_guessed),
        get_color_for_letter("w", &prev_letters_guessed),
        get_color_for_letter("x", &prev_letters_guessed),
        get_color_for_letter("y", &prev_letters_guessed),
        get_color_for_letter("z", &prev_letters_guessed),
    );

    let guess = Text::new("➠")
        // inquire::set_global_render_config(get_render_config());
        // render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightRed);
        // .with_help_message(&format!(
        //     "{} {}",
        //     style("Chyeah").cyan().on_blue(),
        //     style("booooiiiiiiiii").red().on_yellow()
        // ))
        .with_help_message(&hardcodeded_letters_remaining)
        .with_validator(required!())
        .with_validator(length!(letters_in_word))
        // .with_validator(is_valid_english_word())
        // .with
        .with_validator(move |word: &str| {
            // if *val <= 0.0f64 {
            //     Ok(Validation::Invalid(
            //         "You must donate a positive amount of dollars".into(),
            //     ))
            // } else {
            //     Ok(Validation::Valid)
            // }

            println!("");
            println!("");
            println!(
                "{}",
                format!("Deciding if {word} is a valid english word...")
            );

            let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

            symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
            // symspell.load_bigram_dictionary(
            //     "./data/frequency_bigramdictionary_en_243_342.txt",
            //     0,
            //     2,
            //     " ",
            // );

            let suggestions = symspell.lookup(&word, symspell::Verbosity::Top, 2);
            // println!("{:?}", suggestions);

            if (suggestions.len() > 0 && suggestions[0].distance == 0)
                || word == secret_word.clone()
            {
                return Ok(Validation::Valid);
            }

            Ok(Validation::Invalid(
                format!("\n{} is not a valid english word.", word).into(),
            ))
        })
        // .with_validator(is_english_word_validator)
        // .render_config(RenderConfig::from(RenderConfig::default()).with_default_value())
        .prompt()
        .unwrap();

    let (new_letters_guessed, guessed_correctly) = get_new_letters_guessed(
        guess.clone(),
        secret.clone(),
        letters_in_word,
        prev_letters_guessed,
    );

    if guessed_correctly {
        print_you_win(guesses_already_made);

    // if (perfect_match) {
    // console_printer::print_you_win(remaining_guesses)
    } else if remaining_guesses == 1 {
        print_you_lose()
    } else {
        let scored_guess = print_scored_guess(&guess, &new_letters_guessed);
        scored_guesses.push(format!("Guess #{guesses_already_made}) {scored_guess}"));

        // println!("Scored guess... {}", scored_guess);

        for prev_scored_guess in &scored_guesses {
            println!("{prev_scored_guess}");
        }

        prompt_for_guess(
            remaining_guesses - 1,
            secret,
            letters_in_word,
            new_letters_guessed,
            guesses_already_made + 1,
            scored_guesses,
        );
    }

    // match name {
    //     Ok(name) => println!("Hello {}", name),
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }

    // prompt.split_at(3);
    //         style(first_three)
    //         .blue()
    //         .bold()
    //         .to_string()
    //         .bold()
    //         .blue()
    //         .on_black()
    //         + &style(rest).red().bold().to_string().bold().red().on_black()

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

fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("").with_fg(inquire::ui::Color::LightRed);
    render_config.highlighted_option_prefix =
        Styled::new("➠").with_fg(inquire::ui::Color::LightYellow);
    render_config.selected_checkbox = Styled::new("☑").with_fg(inquire::ui::Color::LightGreen);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");
    render_config.answered_prompt_prefix = Styled::new("You guessed:");

    let foo = Styled::new("$").with_fg(inquire::ui::Color::LightRed);

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(inquire::ui::Color::LightRed));

    // render_config.answer = StyleSheet::new()
    // .with_attr(Attributes::ITALIC)
    // .with_fg(Color::LightYellow)
    // .;

    render_config.help_message = RenderConfig::default().prompt;

    render_config
}

fn get_color_for_letter(
    letter: &str,
    letters_guessed: &HashMap<String, GuessState>,
) -> StyledObject<String> {
    match letters_guessed.get(letter).unwrap() {
        GuessState::In_Word_Found_Location(_) => style(letter.to_string()).magenta().on_green(),
        GuessState::In_Word_Unknown_Location => style(letter.to_string()).magenta().on_yellow(),
        GuessState::Guessed_Not_In_Word => style(letter.to_string()).black().on_black(),
        GuessState::Unguessed => style(letter.to_string()).black().on_white(),
    }
}

fn get_color_for_guessed_state(text: &str, guessed_state: &GuessState) -> StyledObject<String> {
    match guessed_state {
        GuessState::In_Word_Found_Location(_) => style(text.to_string()).magenta().on_green(),
        GuessState::In_Word_Unknown_Location => style(text.to_string()).magenta().on_yellow(),
        GuessState::Guessed_Not_In_Word => style(text.to_string()).black().on_black(),
        GuessState::Unguessed => style(text.to_string()).black().on_white(),
    }
}

fn get_new_letters_guessed(
    guess: String,
    secret_word: String,
    letters_in_word: usize,
    mut prev_letters_guessed: HashMap<String, GuessState>,
) -> (HashMap<String, GuessState>, bool) {
    for i in 0..letters_in_word {
        let guess_char = guess.chars().nth(i.into()).unwrap();
        let secret_char = secret_word.chars().nth(i.into()).unwrap();

        println!("Comparing char {i} of guess {guess_char} to {secret_char}");

        let prev_guessed_state = prev_letters_guessed.get(&guess_char.to_string()).unwrap();

        // match prev_guessed_state {
        //     GuessState::In_Word_Unknown_Location | GuessState::Unguessed => {
        if guess_char == secret_char {
            prev_letters_guessed
                .insert(
                    guess_char.to_string(),
                    GuessState::In_Word_Found_Location(i as u8),
                )
                .unwrap();
        } else if secret_word.contains(guess_char) {
            prev_letters_guessed
                .insert(guess_char.to_string(), GuessState::In_Word_Unknown_Location)
                .unwrap();
        } else {
            prev_letters_guessed
                .insert(guess_char.to_string(), GuessState::Guessed_Not_In_Word)
                .unwrap();
        }
        // }
        // default => (),
        // }
    }

    (prev_letters_guessed, guess == secret_word)
}

fn check_for_green() {}
