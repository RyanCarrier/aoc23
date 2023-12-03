use std::collections::HashSet;

use crate::util::{self, Problem};

pub const DAY3: Problem = Problem {
    day: 3,
    part1,
    part2,
    test_data: Some(test_data),
};
const SYMBOLS: [char; 15] = [
    '#', '$', '*', '+', '!', '@', '%', '^', '(', ')', '_', '-', '&', '/', '=',
];

pub fn part1(lines: &Vec<String>) -> String {
    let mut is_number = false;
    let mut number_index = 0;
    let mut number_len = 0;
    let mut touching_symbol = false;
    let mut total = 0;
    let lines = lines
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for i in 0..lines.len() {
        // let l = lines[i].chars().collect::<Vec<char>>();
        let l = &lines[i];
        for j in 0..l.len() + 1 {
            //a little gross but means we can group the logic without annoying
            //closure borrowing or anything
            if j == l.len() || !l[j].is_numeric() {
                //if we have finished the last character OR the next char is not a number
                // (either way we need to check for number being complete)
                if is_number {
                    if touching_symbol {
                        let number = l[number_index..number_index + number_len]
                            .iter()
                            .collect::<String>();
                        let number = number.parse::<i32>().expect("Failed to parse number");
                        total += number;
                    }
                    is_number = false;
                    touching_symbol = false;
                }
            } else if l[j].is_numeric() {
                if is_number {
                    number_len += 1;
                } else {
                    is_number = true;
                    number_index = j;
                    number_len = 1;
                }
            }
            if is_number && !touching_symbol {
                util::TRANSFORMS.iter().for_each(|t| {
                    let new_i = i as isize + t[0];
                    let new_j = j as isize + t[1];
                    if new_i >= 0
                        && new_i < lines.len() as isize
                        && new_j >= 0
                        && new_j < lines[new_i as usize].len() as isize
                    {
                        if SYMBOLS.contains(&lines[new_i as usize][new_j as usize]) {
                            touching_symbol = true;
                        }
                    }
                });
            }
        }
    }
    format!("{}", total)
}

struct GearNumber {
    number: i32,
    gears: Vec<(usize, usize)>,
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut is_number = false;
    let mut number_index = 0;
    let mut number_len = 0;
    let mut gears: Vec<(usize, usize)> = Vec::new();
    let mut numbers: Vec<GearNumber> = Vec::new();
    let lines = lines
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for i in 0..lines.len() {
        let l = &lines[i];
        for j in 0..l.len() + 1 {
            //a little gross but means we can group the logic without annoying
            //closure borrowing or anything
            if j == l.len() || !l[j].is_numeric() {
                if is_number {
                    if !gears.is_empty() {
                        let number = l[number_index..number_index + number_len]
                            .iter()
                            .collect::<String>();
                        let number = number.parse::<i32>().expect("Failed to parse number");
                        numbers.push(GearNumber {
                            number,
                            gears: gears.clone(),
                        });
                    }
                    is_number = false;
                }
            } else if l[j].is_numeric() {
                if is_number {
                    number_len += 1;
                } else {
                    is_number = true;
                    number_index = j;
                    number_len = 1;
                    gears = Vec::new();
                }
            }
            if is_number {
                util::TRANSFORMS.iter().for_each(|t| {
                    let new_i = i as isize + t[0];
                    let new_j = j as isize + t[1];
                    if new_i >= 0
                        && new_i < lines.len() as isize
                        && new_j >= 0
                        && new_j < lines[new_i as usize].len() as isize
                    {
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;
                        if lines[new_i][new_j] == '*' {
                            gears.push((new_i, new_j));
                        }
                    }
                });
            }
        }
    }
    let all_gears: HashSet<(usize, usize)> = numbers.iter().fold(HashSet::new(), |mut v, n| {
        v.extend(n.gears.clone().iter());
        v
    });
    let mut total = 0;
    for g in all_gears {
        let touching: Vec<&GearNumber> = numbers.iter().filter(|n| n.gears.contains(&g)).collect();
        if touching.len() == 2 {
            total += touching[0].number * touching[1].number;
        }
    }
    format!("{}", total)
}
pub fn test_data() -> String {
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
    .to_owned()
}
