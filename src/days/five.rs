use super::read_input;

pub fn run() {
    let input = read_input(5);
    let lines: Vec<&str> = input.lines().collect();

    let mut new = Vec::<i64>::new();
    let mut current = Vec::<i64>::new();

    // parse seeds
    let seeds_str: Vec<&str> = lines[0][7..].split(' ').collect();
    for seed in seeds_str {
        current.push(seed.parse::<i64>().unwrap());
    }

    // convert
    for line in &lines[3..] {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            for item in current {
                new.push(item);
                println!("{} -> {}", item, item);
            }
            println!("end conversion");
            current = new.clone();
            new.clear();
            continue;
        }
        let split: Vec<&str> = line.split(' ').collect();
        let destination: i64 = split[0].parse().unwrap();
        let source: i64 = split[1].parse().unwrap();
        let length: i64 = split[2].parse().unwrap();
        let source_range = source..source + length;

        let mut offset = 0;
        for i in 0..current.len() {
            let thing = current[i - offset];
            if source_range.contains(&thing) {
                new.push(destination + thing - source);
                println!("{} -> {}", thing, destination + thing - source);
                current.remove(i - offset);
                offset += 1;
            }
        }
    }
    for item in current {
        new.push(item);
        println!("{} -> {}", item, item);
    }
    println!("end conversion");
    current = new.clone();
    new.clear();

    let mut lowest = i64::MAX;
    for c in current {
        if c < lowest {
            lowest = c;
        }
    }
    println!("LOWEST: {}", lowest);
}