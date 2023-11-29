use crate::util::Problem;

struct Day0;
struct Day0Data {
    data: Vec<i32>,
}

impl Problem for Day0 {
    fn part1(lines: Vec<String>) -> String {
        let data = import(lines);
    }

    fn part2(lines: Vec<String>) -> String {
        let data = import(lines);
    }
    fn test_data() -> Option<String> {
        Some("".to_owned())
    }
}

fn import(lines: Vec<String>) -> Day0Data {
    Day0Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
