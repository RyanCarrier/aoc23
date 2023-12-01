use crate::util::Problem;

pub const DAY1: Problem = Problem {
    day: 1,
    part1,
    part2,
    test_data,
};

pub fn part1(lines: Vec<String>) -> String {
    let mut total = 0;
    for l in lines {
        let mut num = 0;
        for c in l.chars() {
            if c.is_numeric() {
                num = c.to_string().parse::<i32>().unwrap() * 10;
                break;
            }
        }
        for c in l.chars().rev() {
            if c.is_numeric() {
                num += c.to_string().parse::<i32>().unwrap();
                break;
            }
        }
        total += num;
    }
    format!("{}", total)
}

pub fn part2(lines: Vec<String>) -> String {
    return "".to_owned();
}
pub fn test_data() -> Option<String> {
    Some(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_owned(),
    )
}

// fn import(lines: Vec<String>) -> Day0Data {
//probably not needed half the time
// Day0Data {
//     data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
// }
// }
