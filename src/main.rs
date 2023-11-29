//i don't recall what this was for
use ansi_term::Style;
use std::time::{Duration, Instant};
pub mod day0;
mod util;

struct Args {
    day: usize,
    part: usize,
    test: bool,
    benchmark: bool,
}

fn main() {
    let run_benchmark = !true;
    let fns = vec![[day1::part1, day1::part2]];
    if run_benchmark {
        benchmark(fns);
        return;
    }
    let day = fns.len();
    run_specific(fns, day);
    // run_bench(fns, 13, 2000);
}

fn run_specific(fns: Vec<[fn(Vec<String>) -> String; 2]>, n: usize) {
    let start = Instant::now();
    println!(
        "day{}part{}test:\t{}",
        n,
        1,
        fns[n - 1][0](util::get_test_from_file(n))
    );
    let test_duration = start.elapsed();
    println!(
        "day{}part{}:\t{}",
        n,
        1,
        fns[n - 1][0](util::get_from_file(n))
    );
    let part1_duration = start.elapsed() - test_duration;
    println!(
        "day{}part{}:\t{}",
        n,
        2,
        fns[n - 1][1](util::get_from_file(n))
    );
    let total_duration = start.elapsed();
    println!(
        "Completed in {}\t(p1t:{}, p1:{}, p2:{})",
        util::format_duration(total_duration),
        util::format_duration(test_duration),
        util::format_duration(part1_duration),
        util::format_duration(total_duration - test_duration - part1_duration)
    );
}

#[allow(dead_code)]
fn run_bench(fns: Vec<[fn(Vec<String>) -> String; 2]>, n: usize, repeats: usize) {
    let input = util::get_from_file(n);
    let inputs: Vec<Vec<String>> = (0..repeats).map(|_| input.clone()).collect();
    let total_start = Instant::now();
    for input in inputs {
        let _ = fns[n - 1][0](input);
    }
    let duration1 = total_start.elapsed();

    let inputs: Vec<Vec<String>> = (0..repeats).map(|_| input.clone()).collect();
    let start2 = Instant::now();
    for input in inputs {
        let _ = fns[n - 1][1](input);
    }
    let duration2 = start2.elapsed();
    let total_duration = duration1 + duration2;
    println!("Part1 in {}μs", duration1.as_micros() / repeats as u128);
    println!("Part2 in {}μs", duration2.as_micros() / repeats as u128);
    println!(
        "Completed in {}μs",
        total_duration.as_micros() / repeats as u128
    );
    println!(
        "p1 result:{}, p2 result:{}",
        fns[n - 1][0](input.clone()),
        fns[n - 1][1](input.clone())
    );
}

fn benchmark(fns: Vec<[fn(Vec<String>) -> String; 2]>) {
    let mut data = vec![vec![String::new()]; fns.len()];
    for day in 0..fns.len() {
        data[day] = util::get_from_file(day + 1);
    }
    let mut durations = vec![Duration::default(); fns.len()];

    let total_start = Instant::now();
    for day in 0..fns.len() {
        let part1 = data[day].clone();
        let part2 = data[day].clone();
        let start = Instant::now();
        fns[day][0](part1);
        fns[day][1](part2);
        durations[day] = start.elapsed();
    }
    let total_duration = total_start.elapsed();
    println!("{}", Style::new().bold().paint("Day durations"));
    for day in 0..fns.len() {
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
