use std::fs;

pub fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read file")
}
