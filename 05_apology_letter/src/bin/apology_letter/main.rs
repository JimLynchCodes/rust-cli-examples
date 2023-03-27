//!   Notice how we have extracted the logic in this  to make it simple and clean.
//!
//!   The args_reader and file_manager files (which cargo turns into modules) are thin
//!
//!   wrappers around side effect third-party code and so we don't have any unit tests for them.
//!
//!   The "placeholder_replacer" is a pure function that deals with strings and so is easy to unit test.
//!
//!   In the /tests folder we have an integration test which runs the program, overwriting the existing example_output.txt file,
//!
//!   and then reads in the created file to assert that it contains the correct text.

mod args_reader;
mod file_manager;
mod placeholder_replacer;
use args_reader::read_args;
use file_manager::{read_input_file, write_to_txt_file};
use placeholder_replacer::replace;

fn main() {
    let args = read_args();

    let content = read_input_file();

    let replaced_document_text: String = replace(&content, &args);

    write_to_txt_file(replaced_document_text);
}
