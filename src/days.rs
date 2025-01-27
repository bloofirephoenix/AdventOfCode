use std::fs;

pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;
pub mod eight;
pub mod nine;
pub mod ten;
pub mod eleven;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/{}.txt", day))
        .expect("Failed to read input")
}

pub fn run_day(day: impl Day, part2: bool) {
    if part2 {
        day.part2();
    } else {
        day.part1();
    }
}

pub trait Day {
    fn part1(self);
    fn part2(self);
}