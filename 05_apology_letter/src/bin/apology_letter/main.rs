mod args_reader;
mod file_manager;
mod placeholder_replacer;
use args_reader::{read_args, Cli};
use file_manager::{read_input_file, write_to_txt_file};
use placeholder_replacer::replace;


fn main() {

    let args = read_args();

    let content = read_input_file();

    let replaced_document_text: String = replace(&content, &args);

    // println!("{}", replaced_document_text);

    write_to_txt_file(replaced_document_text);
}
