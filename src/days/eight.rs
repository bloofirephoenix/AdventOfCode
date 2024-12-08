use std::collections::HashSet;
use gcd::Gcd;

use super::{read_input, Day};

pub struct DayEight;
impl Day for DayEight {
    fn part1(self) {
        let (antennas, bounds) = read();

        let mut antis = HashSet::<(i32,i32)>::new();

        for a in &antennas {
            for b in &antennas {
                if a.pos == b.pos || a.frequency != b.frequency {
                    continue;
                }

                let change = (b.pos.0 - a.pos.0, b.pos.1 - a.pos.1);

                let anti1 = (b.pos.0 + change.0, b.pos.1 + change.1);
                let anti2 = (a.pos.0 - change.0, a.pos.1 - change.1);
                
                if anti1.0 >= 0 && anti1.0 < bounds.0 && anti1.1 >= 0 && anti1.1 < bounds.1 {
                    antis.insert(anti1);
                }

                if anti2.0 >= 0 && anti2.0 < bounds.0 && anti2.1 >= 0 && anti2.1 < bounds.1 {
                    antis.insert(anti2);
                }
            }
        }

        println!("ANSWER: {}", antis.len())
    }

    fn part2(self) {
        let (antennas, bounds) = read();

        let mut antis = HashSet::<(i32,i32)>::new();

        for a in &antennas {
            for b in &antennas {
                if a.pos == b.pos || a.frequency != b.frequency {
                    continue;
                }

                let change = (b.pos.0 - a.pos.0, b.pos.1 - a.pos.1);

                let d = ((change.0).abs() as u32).gcd(change.1.abs() as u32) as i32;

                let change = (change.0 / d, change.1 / d);

                let mut pos = (a.pos.0, a.pos.1);
                while pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1 {
                    antis.insert(pos);
                    pos = (pos.0 - change.0, pos.1 - change.1);
                }

                let mut pos = (a.pos.0, a.pos.1);
                while pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1 {
                    antis.insert(pos);
                    pos = (pos.0 + change.0, pos.1 + change.1);
                }
            }
        }

        println!("ANSWER: {}", antis.len())
    }
}

#[derive(Debug)]
struct Antenna {
    pos: (i32, i32),
    frequency: char,
}

/// Returns antennas, bounds
fn read() -> (Vec<Antenna>, (i32, i32)) {
    let input = read_input(8);
    let mut antennas = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        x = 0;
        for c in line.chars() {
            if c != '.' {
                antennas.push(Antenna {
                    pos: (x,y),
                    frequency: c
                });
            }
            x += 1;
        }
        y += 1;
    }

    (antennas, (x, y))
}
