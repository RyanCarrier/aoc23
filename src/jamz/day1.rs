use std::collections::HashMap;

use crate::util::Problem;
pub const DAY1: Problem = Problem {
    day: 1,
    part1,
    part2,
    test_data: || None,
};

fn part1(lines: &Vec<String>) -> String {
    let lines: Vec<&str> = lines
        .iter()
        .map(|l| l as &str)
        .filter(|l| !l.is_empty())
        .collect();
    return lines[0].to_owned();
    // solve_a(lines).to_string()
}
fn part2(lines: &Vec<String>) -> String {
    let lines: Vec<&str> = lines
        .iter()
        .map(|l| l as &str)
        .filter(|l| !l.is_empty())
        .collect();

    solve(lines).to_string()
}
// Left part A solution for posterity
#[allow(dead_code)] //dead slow code that is lmao gottem
fn solve_a(input: Vec<&str>) -> usize {
    let parse_calibration_value = |line: &str| -> usize {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first_digit = digits
            .next()
            .expect("calibration string should contain a digit");
        let last_digit = digits.last().unwrap_or(first_digit);

        (first_digit * 10 + last_digit) as usize
    };

    input
        .into_iter()
        .map(|line| parse_calibration_value(line))
        .sum()
}

pub fn solve(input: Vec<&str>) -> usize {
    let map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let parse_calibration_value = |line: &str| -> usize {
        let mut digits = Vec::<usize>::new();
        for i in 0..line.len() {
            for (key, value) in &map {
                if line[i..].starts_with(key) {
                    digits.push(*value);
                    break;
                }
            }
        }
        let first_digit = digits
            .first()
            .expect("calibration string should contain a digit");
        let last_digit = digits.last().unwrap_or(first_digit);

        first_digit * 10 + last_digit
    };

    input
        .into_iter()
        .map(|line| parse_calibration_value(line))
        .sum()
}
