use super::read_input;

struct Converter {
    source: i64,
    destination: i64,
    length: i64
}

pub fn run() {
    let input = read_input(5);
    let lines: Vec<&str> = input.lines().collect();

    let mut converters: Vec<Vec<Converter>> = Vec::new();

    // parse converters
    println!("Parsing converters");
    let mut current_converters: Vec<Converter> = Vec::new();
    for line in &lines[3..] {
        if line.contains(':') {
            converters.push(current_converters);
            current_converters = Vec::new();
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let split: Vec<&str> = line.split(' ').collect();
        let destination: i64 = split[0].parse().unwrap();
        let source: i64 = split[1].parse().unwrap();
        let length: i64 = split[2].parse().unwrap();
        let converter = Converter {
            source,
            destination,
            length
        };
        current_converters.push(converter);
    }
    converters.push(current_converters);

    // parse seeds
    println!("Parsing seeds");
    let mut seeds_str: Vec<&str> = lines[0][7..].split(' ').collect();
    seeds_str.reverse();
    let mut lowest = i64::MAX;

    let seed_counter_reset = 10_000_000;
    let mut seed_counter = seed_counter_reset;

    while !seeds_str.is_empty() {
        let first = seeds_str.pop().unwrap().parse::<i64>().unwrap();
        let len = seeds_str.pop().unwrap().parse::<i64>().unwrap();

        for seed in first..first + len {
            let mut current = seed;

            'phase_loop: for phase in &converters {
                'converter_loop: for converter in phase {
                    if current >= converter.source && current < converter.source + converter.length {
                        current = converter.destination + current - converter.source;
                        continue 'phase_loop;
                    }
                }
            }

            if current < lowest {
                lowest = current;
            }
            seed_counter -= 1;

            if seed_counter <= 0 {
                seed_counter = seed_counter_reset;
                println!("{} more seeds processed. lowest={}. seeds left in range={}", seed_counter_reset, lowest, len-(seed-first))
            }
        }
        println!("Finished seed range {}-{}", first, first + len);
    }
    println!("Result: {}", lowest);
}