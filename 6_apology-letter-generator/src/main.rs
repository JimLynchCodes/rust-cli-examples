/*
   This program can be run with the command: `cargo run`

   Note that it requires two arguments, a _recipient_name_ and a _thing_you_did_.

   An example that runs cleanly: `cargo run -- Bob "breaking your coffee table"`

   Check that the output has benn written to "example_output.txt"
*/

use clap::Parser;

/// Search for a recipient name in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The recipient name to look for
    recipient_name: String,
    /// The thing you did to the file to read
    thing_you_did: String,
}

const RECIPIENT_NAME_PLACEHOLDER: &str = "*[__RECIPIENT_NAME__]*";
const THING_YOU_DID_PLACEHOLDER: &str = "*[__THING_YOU_DID__]*";

fn main() {
    let args = Cli::parse();
    
    let content = std::fs::read_to_string("./example_input.txt").expect("could not read file");
    
    let mut new_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let replaced_line = get_replaced_line(line, &args);
        new_lines.push(replaced_line);
    }

    let replaced_document_text: String = new_lines.join("\n");

    println!("{}", replaced_document_text);

    std::fs::write("./example_output.txt", replaced_document_text).unwrap();

}

fn get_replaced_line(line: &str, args: &Cli) -> String {
    if line.contains(&RECIPIENT_NAME_PLACEHOLDER) {
        return replace_string_within_string(
            line,
            &RECIPIENT_NAME_PLACEHOLDER,
            &args.recipient_name,
        );
    } else if line.contains(&THING_YOU_DID_PLACEHOLDER) {
        return replace_string_within_string(line, &THING_YOU_DID_PLACEHOLDER, &args.thing_you_did);
    }
    String::from(line)
}

fn replace_string_within_string(original_line: &str, placeholder: &str, new_value: &str) -> String {
    let first_char_index = original_line.find(placeholder).unwrap();
    let last_char_index = first_char_index + placeholder.chars().count();

    let (beginning, _) = original_line.split_at(first_char_index);
    let (_, end) = original_line.split_at(last_char_index);

    let replaced_line = format!("{}{}{}", beginning, new_value, end);

    return replaced_line;
}
