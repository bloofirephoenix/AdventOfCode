use super::read_input;

pub fn run() {
    let input = read_input(6);
    let lines: Vec<&str> = input.lines().collect();

    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();
    for time in &lines[0].split(' ').collect::<Vec<&str>>()[1..] {
        if time.is_empty() {
            continue;
        }
        times.push(time.parse().unwrap());
    }
    for distance in &lines[1].split(' ').collect::<Vec<&str>>()[1..] {
        if distance.is_empty() {
            continue;
        }
        distances.push(distance.parse().unwrap());
    }

    let mut result = 0;
    for race in 0..times.len() {
        let time = times[race];
        let distance = distances[race];

        let mut ways_to_win = 0;
        for hold_time in 0..=time {
            let distance_traveled = (time - hold_time) * hold_time;
            if distance_traveled > distance {
                ways_to_win += 1;
            }
        }
        if result == 0 {
            result = ways_to_win;
        } else {
            result *= ways_to_win;
        }
    }
    println!("Result: {}", result);
}