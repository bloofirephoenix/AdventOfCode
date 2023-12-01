use super::read_input;

pub fn run() {
    let input = read_input(1);

    let mut number: i32 = 0;
    
    for line in input.lines() {
        let mut line_numbers = Vec::<char>::new();
        for c in line.chars() {
            if c.is_numeric() {
                line_numbers.push(c);
            }
        }
        let num_string = format!("{}{}", line_numbers.first().unwrap(), line_numbers.last().unwrap());
        number += num_string.parse::<i32>().unwrap();
    }

    println!("Result: {}", number);
}