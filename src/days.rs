use std::fs;

pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/{}.txt", day))
        .expect("Failed to read input")
}