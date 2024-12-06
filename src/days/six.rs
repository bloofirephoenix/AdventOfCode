use std::collections::{HashMap, HashSet};

use super::{read_input, Day};

pub struct DaySix;
impl Day for DaySix {
    fn part1(self) {
        let (world, mut guard_pos) = read();
        let mut guard_direction = (0, -1);

        let mut visited = Vec::<(i32,i32)>::new();
        
        loop {
            if !visited.contains(&guard_pos) {
                visited.push(guard_pos);
            }
            let next = (guard_pos.0 + guard_direction.0, guard_pos.1 + guard_direction.1);
            if !world.contains_key(&next) {
                break;
            }

            match world[&next] {
                Location::Free => {
                    guard_pos = next;
                },
                Location::Obstructed => {
                    guard_direction = (-guard_direction.1, guard_direction.0);
                }
            }
        }

        println!("ANSWER: {}", visited.len())
    }

    fn part2(self) {
        let (world, original_guard_pos) = read();

        let mut guard_direction = (0, -1);

        let mut visited = HashSet::<(i32,i32)>::new();
        let mut guard_pos = original_guard_pos;
        loop {
            if !visited.contains(&guard_pos) {
                visited.insert(guard_pos);
            }
            let next = (guard_pos.0 + guard_direction.0, guard_pos.1 + guard_direction.1);
            if !world.contains_key(&next) {
                break;
            }

            match world[&next] {
                Location::Free => {
                    guard_pos = next;
                },
                Location::Obstructed => {
                    guard_direction = (-guard_direction.1, guard_direction.0);
                }
            }
        }
        
        let mut answer = 0;
        for loc in visited {
            if matches!(world[&loc], Location::Obstructed) {
                continue;
            }
            
            let mut path = HashSet::<((i32,i32), (i32, i32))>::new();
            let mut guard_pos = original_guard_pos;
            let mut guard_direction = (0, -1);
            
            loop {
                if !path.contains(&(guard_pos, guard_direction)) {
                    path.insert((guard_pos, guard_direction));
                } else {
                    answer += 1;
                    break;
                }

                let next = (guard_pos.0 + guard_direction.0, guard_pos.1 + guard_direction.1);
                if !world.contains_key(&next) {
                    break;
                }
                match world[&next] {
                    Location::Free => {
                        if next == loc {
                            guard_direction = (-guard_direction.1, guard_direction.0);
                        } else {
                            guard_pos = next;
                        }
                    },
                    Location::Obstructed => {
                        guard_direction = (-guard_direction.1, guard_direction.0);
                    }
                }
            }
        }

        println!("ANSWER: {}", answer)
    }
}

#[derive(Debug, Clone)]
enum Location {
    Obstructed,
    Free
}

fn read() -> (HashMap<(i32, i32), Location>, (i32, i32)) {
    let input = read_input(6);

    let mut map = HashMap::new();
    let mut guard: (i32, i32) = (-1, -1);

    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for c in line.chars() {
            match c {
                '.' => map.insert((x,y), Location::Free),
                '^' => {
                    guard = (x,y);
                    map.insert((x,y), Location::Free)
                },
                '#' => map.insert((x,y), Location::Obstructed),
                _ => panic!("bad input char {c}")
            };
            x += 1;
        }
        y += 1;
    }

    (map, guard)
}