use crate::util::Problem;

pub const DAY9: Problem = Problem {
    day: 9,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day9Data {
    data: Vec<Vec<isize>>,
}
fn expand(data: Vec<isize>) -> Vec<Vec<isize>> {
    let mut history: Vec<Vec<isize>> = vec![data];
    while history[history.len() - 1].iter().any(|x| *x != 0) {
        history.push(
            history[history.len() - 1]
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        );
    }
    history
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let total = data.data.into_iter().fold(0, |acc, data_set| {
        let history = expand(data_set);
        let next_value = history
            .into_iter()
            .rev()
            .fold(0, |acc, h| acc + h[h.len() - 1]);
        acc + next_value
    });
    format!("{:?}", total)
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let total = data.data.into_iter().fold(0, |acc, data_set| {
        let history = expand(data_set);
        let prev_value = history.into_iter().rev().fold(0, |acc, h| h[0] - acc);
        acc + prev_value
    });
    format!("{:?}", total)
}
pub fn test_data() -> &'static str {
    "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
}

fn import(lines: &Vec<String>) -> Day9Data {
    //probably not needed half the time
    Day9Data {
        data: lines
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.trim()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect(),
    }
}
