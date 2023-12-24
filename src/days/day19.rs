use crate::util::Problem;

pub const DAY19: Problem = Problem {
    day: 19,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day19Data {
    data: Vec<i32>,
}

pub fn part1(lines: &Vec<String>) -> String {
    let _ = import(lines);
    "".to_owned()
}

pub fn part2(lines: &Vec<String>) -> String {
    let _ = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    ""
}

fn import(lines: &Vec<String>) -> Day19Data {
    Day19Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
