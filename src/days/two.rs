use super::{read_input, Day};
use std::str::FromStr;

pub struct DayTwo;
impl Day for DayTwo {
    fn part1(self) {
        let mut safe_reports = 0;

        let input = read_input(2);
        for line in input.lines() {

            println!("{line}");

            let mut report = Vec::new();

            for s in line.split(' ') {
                report.push(i32::from_str(s).unwrap());
            }
            
            if evaluate_report(&report) {
                safe_reports += 1;
            }
        }

        println!("ANSWER: {safe_reports}");
    }

    fn part2(self) {
        let mut safe_reports = 0;

        let input = read_input(2);
        for line in input.lines() {

            println!("{line}");

            let mut report = Vec::new();

            for s in line.split(' ') {
                report.push(i32::from_str(s).unwrap());
            }
            
            if evaluate_report(&report) {
                safe_reports += 1;
            } else {
                //println!("unsafe?");
                'a: for i in 0..report.len() {
                    let mut r = report.clone();
                    r.remove(i);
                    if evaluate_report(&r) {
                        //println!("safe without i={i}");
                        safe_reports += 1;
                        break 'a;
                    }
                }
            }
        }

        println!("ANSWER: {safe_reports}");
    }
}

fn evaluate_report(report: &Vec<i32>) -> bool {
    let n1 = report[0];
    let n2 = report[1];

    let d = (n1-n2).abs();
    if d < 1 || d > 3 {
        return false;
    }

    let decreasing: bool;
    if n1 > n2 {
        decreasing = true;
    } else if n1 < n2 {
        decreasing = false
    } else {
        return false;
    }
    
    let mut last_n = n2;
    for i in 2..report.len() {
        let n = report[i];

        if decreasing {
            if last_n <= n {
                return false;
            }
        } else {
            if last_n >= n {
                return false;
            }
        }
        
        let d = (n-last_n).abs();
        if d < 1 || d > 3 {
            return false;
        }

        last_n = n;
    }

    true
}