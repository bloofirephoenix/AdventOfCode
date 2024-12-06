use std::{collections::HashMap, str::FromStr};

use crate::days::read_input;

use super::Day;

pub struct DayFive;
impl Day for DayFive {
    fn part1(self) {
        let (rules, lists) = read();

        let mut answer = 0;

        'l: for list in lists {
            let mut previous = list[0];
            for i in 1..list.len() {
                let val = list[i];

                if matches!(rules[&val][&previous], SortOrder::GoesAfter) {
                    continue 'l;
                }
                previous = val;
            }
            answer += list[list.len() / 2]
        }

        println!("ANSWER: {answer}")
    }

    fn part2(self) {
        let (rules, lists) = read();

        let mut answer = 0;

        for list in lists {
            let mut sorted = Vec::<i32>::new();
            'v: for val in &list {
                if sorted.is_empty() {
                    sorted.push(*val);
                    continue;
                }

                for i in 0..sorted.len() {
                    if matches!(rules[&val][&sorted[i]], SortOrder::GoesAfter) {
                        sorted.insert(i, *val);
                        continue 'v;
                    }
                }
                sorted.push(*val);
            }
            if sorted != list {
                answer += sorted[sorted.len() / 2]
            }
        }

        println!("ANSWER: {answer}")
    }
}

fn read() -> (HashMap<i32, HashMap<i32, SortOrder>>, Vec<Vec<i32>>) {
    let input = read_input(5);
    let mut lists = Vec::new();
    let mut rules = HashMap::new();

    let mut reading_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let r: Vec<&str> = line.split('|').collect();
            let lesser = i32::from_str(r[0]).unwrap();
            let greater = i32::from_str(r[1]).unwrap();

            if !rules.contains_key(&lesser) {
                rules.insert(lesser, HashMap::new());
            }
            if !rules.contains_key(&greater) {
                rules.insert(greater, HashMap::new());
            }

            rules.get_mut(&lesser).unwrap().insert(greater, SortOrder::GoesAfter);
            rules.get_mut(&greater).unwrap().insert(lesser, SortOrder::ComesBefore);
        } else {
            let r: Vec<&str> = line.split(',').collect();
            let mut list: Vec<i32> = Vec::new();
            for s in r {
                list.push(i32::from_str(s).unwrap());
            }
            lists.push(list);
        }
    }

    (rules, lists)
}

#[derive(Debug)]
enum SortOrder{
    ComesBefore,
    GoesAfter
}