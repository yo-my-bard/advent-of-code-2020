use std::fs;

pub fn fetch_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Input File not found??")
}
