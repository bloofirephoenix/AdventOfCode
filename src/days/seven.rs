use std::str::FromStr;

use super::{read_input, Day};

pub struct DaySeven;
impl Day for DaySeven {
    fn part1(self) {
        let equations = read();
        let mut answer = 0;
        for equation in equations {
            if does_work(&equation, 0, 0, false) {
                answer += equation.target;
            }
        }

        println!("ANSWER {answer}");
    }

    fn part2(self) {
        let equations = read();
        let mut answer = 0;
        for equation in equations {
            if does_work(&equation, 0, 0, true) {
                answer += equation.target;
            }
        }

        println!("ANSWER {answer}");
    }
}

fn does_work(equation: &Equation, current: u128, index: usize, concat: bool) -> bool {
    if index == equation.numbers.len() {
        return current == equation.target;
    }

    if current > equation.target {
        return false;
    }

    let next = equation.numbers[index];
    does_work(equation, current + next, index + 1, concat) || 
        does_work(equation, current * next, index + 1, concat) ||
        if concat {
            does_work(equation, current * 10_u128.pow((next as f64).log10().floor() as u32 + 1) + next, index + 1, concat)   
        } else {
            false
        }
}

struct Equation {
    target: u128,
    numbers: Vec<u128>
}

fn read() -> Vec<Equation> {
    let input = read_input(7);

    let mut equations = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        let target = u128::from_str(&split[0][..split[0].len()-1]).unwrap();
        
        let mut numbers: Vec<u128> = Vec::new();
        for i in 1..split.len() {
            numbers.push(u128::from_str(&split[i]).unwrap());
        }

        equations.push(Equation {
            target,
            numbers
        });
    }

    equations
}