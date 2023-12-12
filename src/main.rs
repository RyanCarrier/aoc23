//i don't recall what this was for
use ansi_term::Style;
use clap::Parser;
use std::time::{Duration, Instant};
use util::Problem;

pub mod days;
pub mod jamz;
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
    #[arg(short, long, default_value_t = 1_000)]
    iterations: usize,
    #[arg(short, long)]
    virgin: bool,
}
const YEAR: usize = 2023;
const DAYS: [Problem; 11] = [
    days::day1::DAY1,
    days::day2::DAY2,
    days::day3::DAY3,
    days::day4::DAY4,
    days::day5::DAY5,
    days::day6::DAY6,
    days::day7::DAY7,
    days::day8::DAY8,
    days::day9::DAY9,
    days::day10::DAY10,
    days::day11::DAY11,
    days::day12::DAY12,
];
const JAMZ: [Problem; 1] = [jamz::day1::DAY1];

fn main() {
    let args = Args::parse();
    if args.benchmark {
        benchmark(args);
        return;
    }
    let problems = if args.virgin {
        JAMZ.to_vec()
    } else {
        DAYS.to_vec()
    };
    if args.day == 0 {
        //just assume
        for day in 1..=problems.len() {
            run_specific(&problems[day - 1], &args);
        }
        return;
    }
    run_specific(&problems[args.day - 1], &args);
}

fn run_specific(problem: &Problem, args: &Args) {
    println!(
        "{}",
        Style::new()
            .bold()
            .paint("=== Day ".to_owned() + &problem.day.to_string() + " ==="),
    );
    let input = if args.test {
        (problem
            .test_data
            .expect("Asked for test data, but there was none set"))()
        .split('\n')
        .map(|x| x.to_owned())
        .collect()
    } else {
        util::get_input_data(YEAR, problem.day)
    };
    let start = Instant::now();
    if args.part == 0 || args.part == 1 {
        print_result(problem, args.virgin, 1, args.test, (problem.part1)(&input));
    }
    let part1_duration = start.elapsed();
    if args.part == 0 || args.part == 2 {
        print_result(problem, args.virgin, 2, args.test, (problem.part2)(&input));
    }
    let total_duration = start.elapsed();
    println!(
        "Completed in {}\t(p1:{}, p2:{})",
        util::format_duration(total_duration),
        util::format_duration(part1_duration),
        util::format_duration(total_duration - part1_duration)
    );
}
fn print_result(problem: &Problem, virgin: bool, part: usize, test: bool, result: String) {
    println!(
        "day{}part{}{}{}:\t{}",
        problem.day,
        part,
        if virgin { "-JAMZ" } else { "" },
        if test { "-TEST" } else { "" },
        result
    );
}

fn benchmark(args: Args) {
    let runs: usize = args.iterations;
    let problems = if args.virgin {
        println!("Running virgin mode activated");
        JAMZ.to_vec()
    } else {
        println!("Running default high speed computations");
        DAYS.to_vec()
    };
    let range = if args.day == 0 {
        0..problems.len()
    } else {
        (args.day - 1)..(args.day)
    };
    let max = if args.day == 0 {
        problems.len()
    } else {
        args.day
    };
    let mut data = vec![vec![String::new()]; max];
    print!("Getting input data... ");
    for day in range.clone() {
        print!("{}... ", day + 1);
        data[day] = util::get_input_data(YEAR, day + 1);
    }
    println!("");
    println!(
        "{}, {} iterations",
        Style::new().bold().paint("Day durations"),
        runs
    );
    println!("Day\t\tPart1\tPart2\tTotal");
    let mut part1_durations = vec![Duration::default(); max];
    let mut part2_durations = vec![Duration::default(); max];
    let mut total_duration = Duration::default();
    for day in range.clone() {
        let start = Instant::now();
        for _ in 0..runs {
            (problems[day].part1)(&data[day]);
        }
        part1_durations[day] = start.elapsed().div_f64(runs as f64);
        let start = Instant::now();
        for _ in runs..(runs * 2) {
            (problems[day].part2)(&data[day]);
        }
        part2_durations[day] = start.elapsed().div_f64(runs as f64);
        println!(
            "Day {}:\t\t{}\t{}\t{}",
            day + 1,
            util::format_duration(part1_durations[day]),
            util::format_duration(part2_durations[day]),
            Style::new().bold().paint(util::format_duration(
                part1_durations[day] + part2_durations[day]
            ))
        );
        total_duration += part1_durations[day] + part2_durations[day];
    }
    println!(
        "{}",
        Style::new()
            .bold()
            .paint(format!("Total:\t{}", util::format_duration(total_duration)))
    );
}
