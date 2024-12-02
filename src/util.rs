use std::fs;

pub(crate) fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

#[macro_export]
macro_rules! read_lines {
    ($l:tt) => {read_input($l).lines().filter(|x| !x.is_empty())}
}


pub(crate) fn next_u32<'a, I: Iterator<Item = &'a str>>(mut iter: I) -> u32 {
    iter.next().unwrap().parse().unwrap()
}
