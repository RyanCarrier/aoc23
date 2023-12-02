use crate::util::Problem;

pub const DAY1: Problem = Problem {
    day: 1,
    part1,
    part2,
    test_data,
};

pub fn part1(lines: &Vec<String>) -> String {
    let mut total = 0;
    for l in lines {
        for c in l.chars() {
            if c.is_numeric() {
                total += c.to_string().parse::<i32>().unwrap() * 10;
                break;
            }
        }
        for c in l.chars().rev() {
            if c.is_numeric() {
                total += c.to_string().parse::<i32>().unwrap();
                break;
            }
        }
    }
    format!("{}", total)
}

pub fn part2(lines: &Vec<String>) -> String {
    //look we should break this out to fn's but yolo
    let replace = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total = 0;
    for l in lines {
        let chars: Vec<char> = l.chars().collect();
        'first: for i in 0..chars.len() {
            if chars[i].is_numeric() {
                total += chars[i].to_string().parse::<i32>().unwrap() * 10;
                break;
            }
            for j in 0..replace.len() {
                if l[i..].starts_with(replace[j]) {
                    total += (j + 1) as i32 * 10;
                    break 'first;
                }
            }
        }
        'second: for i in (0..chars.len()).rev() {
            if chars[i].is_numeric() {
                total += chars[i].to_string().parse::<i32>().unwrap();
                break;
            }
            for j in 0..replace.len() {
                if l[i..].starts_with(replace[j]) {
                    total += (j + 1) as i32;
                    break 'second;
                }
            }
        }
    }
    format!("{}", total)
}
pub fn test_data() -> Option<String> {
    Some(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_owned(),
    )
}

// fn import(lines: Vec<String>) -> Day0Data {
//probably not needed half the time
// Day0Data {
//     data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
// }
// }
