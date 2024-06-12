use std::collections::HashMap;

use crate::util::Problem;

pub const DAY14: Problem = Problem {
    day: 14,
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &Vec<String>) -> String {
    let lines = lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let len = lines[0].len();
    let max_score = lines.len();
    let mut last_hard = vec![0; len];
    let mut result = 0;
    for i in 0..lines.len() {
        for j in 0..len {
            match lines[i][j] {
                'O' => {
                    result += max_score - last_hard[j];
                    last_hard[j] += 1;
                }
                '#' => last_hard[j] = i + 1,
                _ => {}
            }
        }
    }
    format!("{:?}", result)
}
#[allow(dead_code)]
fn print(lines: &Vec<Vec<u8>>) {
    println!("======");
    for line in lines {
        println!(
            "{}",
            line.iter()
                .map(|x| match x {
                    b'O' => 'O',
                    b'#' => '#',
                    _ => '.',
                })
                .collect::<String>()
        );
    }
    println!("++++++");
}

fn spin(lines: &mut Vec<Vec<u8>>) {
    let len = lines[0].len();
    let max_score = lines.len();
    const ROCK: u8 = b'O';
    const THING: u8 = b'#';
    const EMPTY: u8 = b'.';
    //i was going to make this shorter and nicer... but nah

    // for nw in [true, false] {
    //     for i_rock in [true, false] {}
    // }
    for j in 0..max_score {
        let mut rock_position = 0;
        for i in 0..len {
            match lines[i][j] {
                ROCK => {
                    if rock_position != i {
                        lines[rock_position][j] = ROCK;
                        lines[i][j] = EMPTY;
                    }
                    rock_position += 1;
                }
                THING => rock_position = i + 1,
                _ => {}
            }
        }
    }
    for i in 0..len {
        let mut rock_position = 0;
        for j in 0..max_score {
            match lines[i][j] {
                ROCK => {
                    if rock_position != j {
                        lines[i][rock_position] = ROCK;
                        lines[i][j] = EMPTY;
                    }
                    rock_position += 1;
                }
                THING => rock_position = j + 1,
                _ => {}
            }
        }
    }
    for j in 0..max_score {
        let mut rock_position = max_score - 1;
        for i in (0..len).rev() {
            match lines[i][j] {
                ROCK => {
                    if rock_position != i {
                        lines[rock_position][j] = ROCK;
                        lines[i][j] = EMPTY;
                    }
                    rock_position = if rock_position == 0 {
                        0
                    } else {
                        rock_position - 1
                    };
                }
                THING => rock_position = if i == 0 { 0 } else { i - 1 },
                _ => {}
            }
        }
    }
    for i in 0..len {
        let mut rock_position = max_score - 1;
        for j in (0..max_score).rev() {
            match lines[i][j] {
                ROCK => {
                    if rock_position != j {
                        lines[i][rock_position] = ROCK;
                        lines[i][j] = EMPTY;
                    }
                    rock_position -= 1;
                }
                THING => rock_position = j - 1,
                _ => {}
            }
        }
    }
}

pub fn part2(lines: &Vec<String>) -> String {
    const ROCK: u8 = b'O';
    const CYCLES: usize = 1_000_000_000;
    let mut lines: Vec<Vec<u8>> = lines
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut started: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    let mut cycle = 0;
    let mut ride_it_out = false;
    while cycle < CYCLES {
        let mut new_lines = lines.clone();
        if !ride_it_out && started.contains_key(&lines) {
            let cycle_len = cycle - started[&lines];
            let mult = (CYCLES - cycle) / cycle_len;
            //this could be shorthanded but this makes sure we don't flow past CYCLES
            cycle += mult * cycle_len;
            ride_it_out = true;
            continue;
        }
        spin(&mut new_lines);
        started.insert(lines.clone(), cycle);
        cycle += 1;
        lines = new_lines;
    }
    let max_score = lines.len();
    let result = lines.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + (max_score - i) * x.iter().filter(|&x| *x == ROCK).count()
    });
    format!("{:?}", result)
}
pub fn test_data() -> &'static str {
    "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
}
