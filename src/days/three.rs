use crate::days::read_input;

pub fn run() {
    let input = read_input(3);

    let mut engine: Vec<Vec<char>> = Vec::new(); // engine[y][x] = char

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        engine.push(row);
    }

    let mut digits: Vec<(i32, i32)> = Vec::new();

    let mut result = 0;

    for y in 0..engine.len() {
        for x in 0..(&engine[y]).len() {
            let symbol = engine[y][x];
            if symbol.is_numeric() || symbol == '.' {
                continue;
            }
            // its a symbol!
            let mut adjacent_numbers = Vec::<i32>::new();
            for xc in -1..=1 {
                for yc in -1..=1 {
                    let x = x as i32 + xc;
                    let y = y as i32 + yc;

                    if y < 0 || y >= engine.len() as i32 {
                        continue;
                    }

                    if x < 0 || x >= engine[y as usize].len() as i32 {
                        continue;
                    }

                    if engine[y as usize][x as usize].is_numeric() && !digits.contains(&(x, y)) {
                        adjacent_numbers.push(get_value(x, y, &engine, &mut digits));
                    }
                }
            }

            if adjacent_numbers.len() == 2 {
                result += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }

    println!("RESULT: {}", result);
}

fn get_value(x: i32, y: i32, engine: &Vec<Vec<char>>, digits: &mut Vec<(i32, i32)>) -> i32 {
    let mut string: String = String::from("");
    string += &engine[y as usize][x as usize].to_string();

    // two directions lmao
    for i in (-1..=1).step_by(2) {
        let mut x = x;
        'search_loop: loop {
            x += i;
            if x < 0 || x >= engine[y as usize].len() as i32 {
                break 'search_loop;
            }
            let char = engine[y as usize][x as usize];
            if char.is_numeric() {
                digits.push((x, y));
                if i > 0 {
                    string += &char.to_string();
                } else {
                    string = char.to_string() + &string;
                }
            } else {
                break 'search_loop;
            }
        }
    }

    string.parse().unwrap()
}