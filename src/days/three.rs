use regex::Regex;
use std::str::FromStr;

use super::{read_input, Day};

pub struct DayThree;
impl Day for DayThree {
    fn part1(self) {
        let input = read_input(3);
        let r = Regex::new(r"mul\((\d{1}|\d{2}|\d{3}),(\d{1}|\d{2}|\d{3})\)").unwrap();
        println!("heck");

        let mut result = 0;
        for m in r.captures_iter(&input) {
            let (_, nums) = m.extract::<2>();
            let n1 = i32::from_str(nums[0]).unwrap();
            let n2 = i32::from_str(nums[1]).unwrap();

            result += n1 * n2;
        }

        println!("ANSWER: {result}")
    }

    fn part2(self) {
        let input = read_input(3);
        let r = Regex::new(r"mul\((\d{1}|\d{2}|\d{3}),(\d{1}|\d{2}|\d{3})\)|do\(\)|don't\(\)").unwrap();
        println!("heck");

        let mut result = 0;
        let mut enabled = true;
        for m in r.captures_iter(&input) {
            let command = &m[0];
            if command == "do()" {
                enabled = true;
            } else if command == "don't()" {
                enabled = false;
            } else {
                if enabled {
                    let n1 = i32::from_str(&m[1]).unwrap();
                    let n2 = i32::from_str(&m[2]).unwrap();

                    result += n1 * n2;
                }
            }
        }

        println!("ANSWER: {result}")
    }
}