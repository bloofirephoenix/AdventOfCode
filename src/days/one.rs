use fancy_regex::Regex;

use super::read_input;

pub fn run() {
    let input = read_input(1);
    let regex = Regex::new(r"(?=(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|([1-9]))").unwrap();

    let mut number: i32 = 0;
    
    for line in input.lines() {
        let mut numbers = Vec::<i32>::new();
        for capture in regex.captures_iter(line) {
            if let Ok(capture) = capture {
                for c in capture.iter() {
                    if let Some(c) = c {
                        let num = &line.to_string()[c.range()];
                        if num.is_empty() {
                            continue;
                        }
                        numbers.push(string_to_num(num));
                    }
                }
            }
        }

        let num = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
        number += num.parse::<i32>().unwrap();
    }

    println!("Result: {}", number);
}

fn string_to_num(num: &str) -> i32 {
    match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            num.parse().unwrap()
        }
    }
}