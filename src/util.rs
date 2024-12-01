use std::fs;

pub(crate) fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}
