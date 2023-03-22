const RECIPIENT_NAME_PLACEHOLDER: &str = "*[__RECIPIENT_NAME__]*";
const THING_YOU_DID_PLACEHOLDER: &str = "*[__THING_YOU_DID__]*";

use crate::args_reader::Cli;

pub fn replace(content: &str, args: &Cli) -> String {
    let mut new_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let replaced_line = get_replaced_line(line, &args);
        new_lines.push(replaced_line);
    }

    let replaced_document_text: String = new_lines.join("\n");

    replaced_document_text
}

fn get_replaced_line(line: &str, args: &Cli) -> String {
    if line.contains(RECIPIENT_NAME_PLACEHOLDER) {
        return replace_string_within_string(
            line,
            RECIPIENT_NAME_PLACEHOLDER,
            &args.recipient_name,
        );
    } else if line.contains(THING_YOU_DID_PLACEHOLDER) {
        return replace_string_within_string(line, THING_YOU_DID_PLACEHOLDER, &args.thing_you_did);
    }
    String::from(line)
}

fn replace_string_within_string(original_line: &str, placeholder: &str, new_value: &str) -> String {
    let first_char_index = original_line.find(placeholder).unwrap();
    let last_char_index = first_char_index + placeholder.chars().count();

    let (beginning, _) = original_line.split_at(first_char_index);
    let (_, end) = original_line.split_at(last_char_index);

    let replaced_line = format!("{}{}{}", beginning, new_value, end);

    replaced_line
}

#[cfg(test)]
mod replace_tests {

    use crate::placeholder_replacer::replace;
    use crate::args_reader::Cli;

    #[test]
    fn replaces_vars_in_some_text() {
        
        let mock_cli_input = Cli {
            recipient_name: "a".to_string(),
            thing_you_did: "b".to_string(),
        };

        let initial_with_recipient_name = "foo *[__RECIPIENT_NAME__]* bar";
        let initial_with_thing_you_did = "foo *[__THING_YOU_DID__]* bar";

        let expected_recipient_name_output = "foo a bar";
        let expected_thing_you_did_output = "foo b bar";

        assert_eq!(replace(initial_with_recipient_name, &mock_cli_input), expected_recipient_name_output);
        assert_eq!(replace(initial_with_thing_you_did, &mock_cli_input), expected_thing_you_did_output);
    }

}