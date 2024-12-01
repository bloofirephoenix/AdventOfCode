use std::env;

use days::{one::DayOne, run_day};

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        usage();
        return;
    }

    let part2 = if args.len() == 3 {
        args[2] == "part2"
    } else {
        false
    };
    
    match args[1].as_str() {
        "1" => run_day(DayOne, part2),
        _ => {
            usage();
        },
    }
}

fn usage() {
    println!("args: <day number: int> (part2)")
}
