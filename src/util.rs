use std::fs;

pub(crate) fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub(crate) fn read_no_newline(file_path: &str) -> String {
    read_input(file_path).replace("\n", "")
}

#[macro_export]
macro_rules! read_lines {
    ($l:tt) => {
        read_input($l).lines().filter(|x| !x.is_empty())
    };
}

#[macro_export]
macro_rules! read_lines_u32 {
    ($l:tt) => {
        read_input($l).lines().filter(|x| !x.is_empty()).map(|x| {
            x.split_ascii_whitespace()
                .map(|y| y.parse::<u32>().unwrap())
        })
    };
}

#[allow(unused)]
pub(crate) fn next_u32<'a, I: Iterator<Item = &'a str>>(mut iter: I) -> u32 {
    iter.next().unwrap().parse().unwrap()
}

pub(crate) trait Countable: Iterator {
    fn count_all<F>(self, f: F) -> usize
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.map(f).filter(|&x| x).count()
    }
}

impl<T> Countable for T where T: Iterator {}
