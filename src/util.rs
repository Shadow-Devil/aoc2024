use std::fs;

pub(crate) fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

#[macro_export]
macro_rules! read_lines {
    ($l:tt) => {read_input($l).lines().filter(|x| !x.is_empty())}
}
