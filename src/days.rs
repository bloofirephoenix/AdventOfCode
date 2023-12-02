use std::fs;

pub mod one;
pub mod two;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/{}.txt", day))
        .expect("Failed to read input")
}