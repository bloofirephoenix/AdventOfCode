use super::read_input;

pub fn run() {
    let input = read_input(6);
    let lines: Vec<&str> = input.lines().collect();

    let mut time: String = String::from("");
    let mut distance: String = String::from("");
    for t in &lines[0].split(' ').collect::<Vec<&str>>()[1..] {
        if t.is_empty() {
            continue;
        }
        time += t;
    }
    let time = time.parse::<i64>().unwrap();

    for d in &lines[1].split(' ').collect::<Vec<&str>>()[1..] {
        if d.is_empty() {
            continue;
        }
        distance += d;
    }
    let distance = distance.parse::<i64>().unwrap();

    let mut ways_to_win: i64 = 0;
    for hold_time in 0..=time {
        let distance_traveled = (time - hold_time) * hold_time;
        if distance_traveled > distance {
            ways_to_win += 1;
        }
    }
    println!("Result: {}", ways_to_win);
}