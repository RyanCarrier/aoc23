//i don't recall what this was for
use ansi_term::Style;
use clap::Parser;
use std::time::{Duration, Instant};
use util::Problem;

pub mod day1;
mod util;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: usize,
    #[arg(short, long, default_value_t = 0)]
    part: usize,
    #[arg(short, long)]
    test: bool,
    #[arg(short, long)]
    benchmark: bool,
}
const YEAR: usize = 2023;
const DAYS: [Problem; 1] = [day1::DAY1];

fn main() {
    let args = Args::parse();
    if args.benchmark {
        benchmark(args);
        return;
    }
    if args.day == 0 {
        //just assume
        for day in 1..=DAYS.len() {
            run_specific(&DAYS[day - 1], args.part, args.test);
        }
        return;
    }
    run_specific(&DAYS[args.day - 1], args.part, args.test);
}

fn run_specific(problem: &Problem, part: usize, test: bool) {
    let input = if test {
        (problem.test_data)()
            .unwrap()
            .split('\n')
            .map(|x| x.to_owned())
            .collect()
    } else {
        util::get_input_data(YEAR, problem.day)
    };
    let start = Instant::now();
    if test {
        println!(
            "day{}part1TEST:\t{}",
            problem.day,
            (problem.part1)(input.clone())
        );
        println!(
            "day{}part2TEST:\t{}",
            problem.day,
            (problem.part2)(input.clone())
        );
    }
    let test_duration = start.elapsed();
    if part == 0 || part == 1 {
        println!(
            "day{}part1:\t{}",
            problem.day,
            (problem.part1)(input.clone())
        );
    }
    let part1_duration = start.elapsed() - test_duration;
    if part == 0 || part == 2 {
        println!(
            "day{}part2:\t{}",
            problem.day,
            (problem.part2)(input.clone())
        );
    }
    let total_duration = start.elapsed();
    println!(
        "Completed in {}\t(pt:{}, p1:{}, p2:{})",
        util::format_duration(total_duration),
        util::format_duration(test_duration),
        util::format_duration(part1_duration),
        util::format_duration(total_duration - test_duration - part1_duration)
    );
}

fn benchmark(args: Args) {
    let range = if args.day == 0 {
        0..DAYS.len()
    } else {
        (args.day - 1)..(args.day)
    };
    let max = if args.day == 0 { DAYS.len() } else { args.day };
    let mut data = vec![vec![String::new()]; max];
    for day in range.clone() {
        data[day] = util::get_input_data(YEAR, day + 1);
    }
    let mut durations = vec![Duration::default(); max];

    let total_start = Instant::now();
    for day in range.clone() {
        let part1 = data[day].clone();
        let part2 = data[day].clone();
        let start = Instant::now();
        (DAYS[day].part1)(part1);
        (DAYS[day].part2)(part2);
        durations[day] = start.elapsed();
    }
    let total_duration = total_start.elapsed();
    println!("{}", Style::new().bold().paint("Day durations"));
    for day in range.clone() {
        println!(
            "Day {}:\t{:.1}ms",
            day + 1,
            durations[day].as_micros() as f64 / 1000 as f64
        );
    }
    println!(
        "{}",
        Style::new().bold().paint(format!(
            "Total:\t{}ms",
            total_duration.as_millis().to_string()
        ))
    );
}
