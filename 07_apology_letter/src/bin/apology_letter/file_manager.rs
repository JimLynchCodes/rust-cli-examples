use std::error::Error;

pub fn read_input_file() -> String {
    std::fs::read_to_string("./example_input.txt").expect("could not read file")
}

pub fn write_to_txt_file(replaced_document_text: String) -> Result<(), Box<dyn Error>> {
    Ok(std::fs::write(
        "./example_output.txt",
        replaced_document_text,
    )?)
}
