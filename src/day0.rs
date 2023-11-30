use crate::util::Problem;

pub const DAY0: Problem = Problem {
    day: 0,
    part1,
    part2,
    test_data,
};
struct Day0Data {
    data: Vec<i32>,
}

pub fn part1(lines: Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}

pub fn part2(lines: Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}
pub fn test_data() -> Option<Vec<String>> {
    None
}

fn import(lines: Vec<String>) -> Day0Data {
    //probably not needed half the time
    Day0Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
