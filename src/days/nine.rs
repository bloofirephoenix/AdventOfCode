use super::{read_input, Day};
use std::str::FromStr;

pub struct DayNine;
impl Day for DayNine {
    fn part1(self) {
        let mut blocks = read_blocks();
        rearrange(&mut blocks);
        println!("ANSWER: {}", checksum(&blocks))
    }

    fn part2(self) {
        let (mut sections, mut current_id) = read_sections();
        
        // find the section with that id
        while current_id >= 0 {
            's: for i in 0..sections.len() {
                if sections[i].id != current_id {
                    continue;
                }

                for j in 0..=i {
                    if sections[j].id != -1 || sections[j].size < sections[i].size {
                        continue;
                    }
                    // we have found a suitable free space location
                    let sect = sections.remove(i);

                    sections.insert(i, Section {
                        id: -1,
                        size: sect.size
                    });

                    sections[j].size -= sect.size;
                    if sections[j].size <= 0 {
                        sections.remove(j);
                    }
                    sections.insert(j, sect);
                    break 's;
                }
            }
            current_id -= 1;

            // combine free space sections
            let mut i = 0;
            while i < sections.len() - 1 {
                if sections[i].id == -1 && sections[i + 1].id == -1 {
                    let s = sections.remove(i + 1);
                    sections[i].size += s.size;
                } else {
                    i += 1;
                }
            }
        }

        // convert to blocks
        let mut blocks = Vec::new();

        for section in sections {
            for _ in 0..section.size {
                blocks.push(section.id);
            }
        }

        println!("ANSWER: {}", checksum(&blocks))
    }
}

fn checksum(blocks: &Vec<i32>) -> u128 {
    let mut checksum = 0;

    for i in 0..blocks.len() {
        if blocks[i] == -1 {
            continue;
        }

        checksum += i as u128 * blocks[i] as u128;
    }

    checksum
}

fn rearrange(blocks: &mut Vec<i32>) {
    let mut left = 0;
    let mut right = blocks.len() - 1;
    while left < right {
        // find free space
        while left < blocks.len() && blocks[left] != -1 {
            left += 1;
        }

        // find block
        while right > 0 && blocks[right] == -1 {
            right -= 1;
        }

        if left >= right {
            break;
        }

        blocks[left] = blocks[right];
        blocks[right] = -1;
    }
}

struct Section {
    id: i32,
    size: i32
}

fn read_sections() -> (Vec<Section>, i32) {
    let input = read_input(9);
    let mut files = Vec::new();

    let mut iter = input.chars();
    let mut current_id = 0;
    while let Some(c) = iter.next() {
        let file = i32::from_str(&c.to_string()).unwrap();
        if file > 0 {
            files.push(Section {
                id: current_id,
                size: file
            });
            current_id += 1;
        }

        if let Some(c) = iter.next() {
            let free_space = i32::from_str(&c.to_string()).unwrap();
            files.push(Section {
                id: -1,
                size: free_space
            });
        }
    }

    (files, (current_id - 1).max(0))
}

fn read_blocks() -> Vec<i32> {
    let input = read_input(9);
    let mut blocks = Vec::new();

    let mut iter = input.chars();
    let mut current_id = 0;
    while let Some(c) = iter.next() {
        let file = i32::from_str(&c.to_string()).unwrap();
        for _ in 0..file {
            blocks.push(current_id);
        }
        if file > 0 {
            current_id += 1;
        }

        if let Some(c) = iter.next() {
            let free_space = i32::from_str(&c.to_string()).unwrap();
            for _ in 0..free_space {
                blocks.push(-1);
            }
        }
    }


    blocks
}