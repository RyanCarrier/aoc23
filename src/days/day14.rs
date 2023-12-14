use crate::util::Problem;

pub const DAY14: Problem = Problem {
    day: 14,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day14Data {
    data: Vec<i32>,
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    "".to_owned()
}

pub fn part2(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    ""
}

fn import(lines: &Vec<String>) -> Day14Data {
    Day14Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
