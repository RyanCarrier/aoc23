use std::collections::HashMap;

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
                        let number = number.parse::<usize>().expect("Failed to parse number");
                        total += number;
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
                    touching_symbol = false;
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

pub fn part2(lines: &Vec<String>) -> String {
    let mut is_number = false;
    let mut number_index = 0;
    let mut number_len = 0;
    let mut gears: Vec<(usize, usize)> = Vec::new();
    let lines = lines
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut all_gear_numbers: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
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
                        let number = number.parse::<usize>().expect("Failed to parse number");
                        gears.iter().for_each(|g| {
                            all_gear_numbers
                                .entry(*g)
                                .or_insert(Vec::new())
                                .push(number);
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
                            if !gears.contains(&(new_i, new_j)) {
                                gears.push((new_i, new_j));
                            }
                        }
                    }
                });
            }
        }
    }
    let total = all_gear_numbers
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum::<usize>();
    format!("{}", total)
}
pub fn test_data() -> &'static str {
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
}
