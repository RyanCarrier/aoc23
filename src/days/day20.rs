use crate::util::Problem;

pub const DAY20: Problem = Problem {
    day: 20,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day20Data {
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

fn import(lines: &Vec<String>) -> Day20Data {
    Day20Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
