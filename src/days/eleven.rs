use std::{collections::HashMap, str::FromStr};

use super::{read_input, Day};

// Totally not stolen from Anton :)
// I came up with this all on my own :)
// Anton stole it from ME :)

pub struct DayEleven;
impl Day for DayEleven {
    fn part1(self) {
        let mut rocks = read();
        for i in 0..25 {
            rocks = blink(rocks);
        }
        let mut count = 0;
        for (_, c) in rocks {
            count += c;
        }
        println!("ANSWER: {count}")
    }

    fn part2(self) {
        let mut rocks = read();
        for i in 0..75 {
            rocks = blink(rocks);
        }
        let mut count = 0;
        for (_, c) in rocks {
            count += c;
        }
        println!("ANSWER: {count}")
    }
}

type Rock = u128;

fn blink(rocks: HashMap<Rock, usize>) -> HashMap<Rock, usize> {
    let mut new_rocks = HashMap::new();
    for (value, count) in rocks {
        if value == 0 {
            insert(&mut new_rocks, 1, count);
        } else if digits(value) % 2 == 0 {
            let d = digits(value);
            insert(&mut new_rocks, value % 10_u32.pow(d as u32 / 2) as Rock, count);
            insert(&mut new_rocks, value / 10_u32.pow(d as u32 / 2) as Rock, count);
        } else {
            insert(&mut new_rocks, value * 2024, count);
        }
    }

    new_rocks
}

fn insert(map: &mut HashMap<Rock, usize>, value: Rock, count: usize) {
    map.insert(value, count + *map.get(&value).unwrap_or(&0));
}

fn digits(mut i: Rock) -> Rock {
    let mut digits = 0;
    while i > 0 {
        i /= 10;
        digits += 1;
    }

    digits
}

fn read() -> HashMap<Rock, usize> {
    let input = read_input(11);
    let mut rocks = HashMap::new();

    for rock in input.split(' ') {
        let v = Rock::from_str(rock).unwrap();
        if rocks.contains_key(&v) {
            *rocks.get_mut(&v).unwrap() += 1;
        } else {
            rocks.insert(v, 1);
        }
    }

    rocks
}