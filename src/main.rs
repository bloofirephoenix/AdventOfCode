use std::env;

use days::{eight::DayEight, five::DayFive, four::DayFour, nine::DayNine, one::DayOne, run_day, seven::DaySeven, six::DaySix, ten::DayTen, three::DayThree, two::DayTwo};

mod days;
mod grid;

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
        "2" => run_day(DayTwo, part2),
        "3" => run_day(DayThree, part2),
        "4" => run_day(DayFour, part2),
        "5" => run_day(DayFive, part2),
        "6" => run_day(DaySix, part2),
        "7" => run_day(DaySeven, part2),
        "8" => run_day(DayEight, part2),
        "9" => run_day(DayNine, part2),
        "10" => run_day(DayTen, part2),
        _ => {
            usage();
        },
    }
}

fn usage() {
    println!("args: <day number: int> (part2)")
}
