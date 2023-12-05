use std::collections::VecDeque;

use super::read_input;

pub fn run() {
    let input = read_input(4);
    
    let mut total_scratchcards = 0;

    let lines: Vec<&str> = input.lines().collect();
    let mut cards_to_process = VecDeque::<usize>::new();

    for i in 0..lines.len() {
        cards_to_process.push_back(i);
    }

    while !cards_to_process.is_empty() {
        let card_index = cards_to_process.pop_front().unwrap();
        if card_index >= lines.len() {
            continue;
        }

        let matches = get_matches(lines[card_index]);
        total_scratchcards += 1;

        for a in 1..=matches.len() {
            cards_to_process.push_back(card_index + a);
        }
    }

    println!("RESULT: {}", total_scratchcards);
}

fn get_matches(line: &str) -> Vec<i32> {
    let mut matches = Vec::new();

    let card_split: Vec<&str> = line.split(':').collect();
    let number_split: Vec<&str> = card_split[1].split("|").collect();let mut winning_numbers = Vec::<i32>::new();
    for winning_num in number_split[0].split(' ') {
        if winning_num.is_empty() {
            continue;
        }
        winning_numbers.push(winning_num.parse().unwrap());
    }

    for my_num in number_split[1].split(' ') {
        if my_num.is_empty() {
            continue;
        }
        let num: i32 = my_num.parse().unwrap();
        if winning_numbers.contains(&num) {
            matches.push(num);
        }
    }

    matches
}

fn get_card_value(matches: &Vec<i32>) -> i32 {
    let mut value = 0;
    for m in matches {
        if value == 0 {
            value = 1;
        } else {
            value *= 2;
        }
    }

    value
}