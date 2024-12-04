use std::collections::HashMap;

use super::{read_input, Day};

pub struct DayFour;
impl Day for DayFour {
    fn part1(self) {
        let map = get_input();
        let mut matches = 0;
        for (key, _) in &map {
            matches += find_word(&map, *key, "XMAS", None);
        }

        println!("ANSWER: {matches}");
    }

    fn part2(self) {
        let map = get_input();
        let mut matches = 0;
        for (key, c) in &map {
            if *c == 'A' {
                let mut x_matches = 0;

                for x in (-1..=1).step_by(2) {
                    for y in (-1..=1).step_by(2) {
                        if matches!(map.get(&(key.0 + x, key.1 + y)), Some('M')) {
                            if matches!(map.get(&(key.0 - x, key.1 - y)), Some('S')) {
                                x_matches += 1;
                            }
                        }
                    }
                }

                if x_matches >= 2 {
                    matches += 1
                }
            }
        }

        println!("ANSWER: {matches}");
    }
}

fn find_word(map: &HashMap<(i32, i32), char>, start: (i32, i32), word: &str, direction: Option<(i32, i32)>) -> i32 {
    if word.is_empty() {
        return 1;
    }

    if !map.contains_key(&start) {
        return 0;
    }
    
    if map[&start] == word.chars().next().unwrap() {
        if direction.is_none() {
            let mut result = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    result += find_word(map, (start.0 + x, start.1 + y), &word[1..], Some((x, y)));
                }
            }
            return result;
        } else {
            let direction = direction.unwrap();
            return find_word(map, (start.0 + direction.0, start.1 + direction.1), &word[1..], Some(direction));
        }
    } else {
        return 0
    }
}

fn get_input() -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();

    let input = read_input(4);

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            map.insert((x,y), c);

            x += 1;
        }
        y += 1;
    }

    map
}