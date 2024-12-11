use std::{collections::HashSet, str::FromStr};

use crate::grid::Grid;

use super::{read_input, Day};

pub struct DayTen;
impl Day for DayTen {
    fn part1(self) {
        let grid = read();

        let mut score = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[(x,y)] == 0 {
                    let mut endpoints = HashSet::new();
                    _ = do_trailhead(&grid, x, y, 0, &mut endpoints);
                    score += endpoints.len();
                }
            }
        }

        println!("ANSWER: {score}");
    }

    fn part2(self) {
        let grid = read();

        let mut score = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[(x,y)] == 0 {
                    score += do_trailhead(&grid, x, y, 0, &mut HashSet::new());
                }
            }
        }

        println!("ANSWER: {score}");
    }
}

fn do_trailhead(grid: &Grid<i32>, x: usize, y: usize, value: i32, found_endpoints: &mut HashSet<(usize, usize)>) -> i32 {
    if value == 9 {
        found_endpoints.insert((x,y));
        return 1;
    }

    let mut score = 0;
    for xc in -1_i32..=1 {
        for yc in -1_i32..=1 {
            if (xc != 0 && yc != 0) || (xc == 0 && yc == 0) {
                continue;
            }

            let new_x = x as i32 + xc;
            let new_y = y as i32 + yc;

            if new_x < 0 || new_x >= grid.width() as i32 || new_y < 0 || new_y >= grid.height() as i32 {
                continue;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if grid[(new_x, new_y)] == value + 1 {
                score += do_trailhead(grid, (x as i32 + xc) as usize, (y as i32 + yc) as usize, value + 1, found_endpoints);
            }
        }
    }

    score
}

fn read() -> Grid<i32> {
    let input = read_input(10);
    let lines: Vec<&str> = input.lines().collect();

    let mut grid = Grid::new(lines[0].len(), lines.len());

    let mut y = 0;
    for line in lines {
        let mut x= 0;
        for c in line.chars() {
            if c == '.' { // for debugging :)
                grid[(x,y)] = -1;
            } else {
                grid[(x,y)] = i32::from_str(&c.to_string()).unwrap();
            }
            x += 1;
        }

        y += 1;
    }

    grid
}