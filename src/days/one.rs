use std::collections::HashMap;

use super::{read_input, Day};

pub struct DayOne;
impl Day for DayOne {
    fn part1(self) {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        let input = read_input(1);
        for line in input.lines() {
            let line: Vec<&str> = line.split("   ").collect();
            let l = i32::from_str_radix(line[0], 10).unwrap();
            let r = i32::from_str_radix(line[1], 10).unwrap();
            left.push(l);
            right.push(r);
        }

        left.sort();
        right.sort();

        let mut distance = 0;
        for i in 0..left.len() {
            distance += (left[i] - right[i]).abs()
        }

        println!("ANSWER: {distance}")
    }

    fn part2(self) {
        let mut left: Vec<i32> = Vec::new();
        let mut right: HashMap<i32, i32> = HashMap::new();

        let input = read_input(1);
        for line in input.lines() {
            let line: Vec<&str> = line.split("   ").collect();
            let l = i32::from_str_radix(line[0], 10).unwrap();
            let r = i32::from_str_radix(line[1], 10).unwrap();
            left.push(l);

            if right.contains_key(&r) {
                right.insert(r, right[&r] + 1);
            } else {
                right.insert(r, 1);
            }
        }

        let mut similarity = 0;
        for v in left {
            if right.contains_key(&v) {
                similarity += v * right[&v];
            }
        }

        println!("ANSWER: {similarity}")
    }
}