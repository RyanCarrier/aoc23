use std::collections::{HashSet, VecDeque};

use crate::util::Problem;

pub const DAY18: Problem = Problem {
    day: 18,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Instruction {
    direction: u8,
    distance: isize,
}
struct Day18Data {
    data: Vec<Instruction>,
}
impl Day18Data {
    fn make_path(&self) -> (Vec<(usize, usize)>, (usize, usize)) {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
        let (mut y, mut x) = (0_isize, 0_isize);
        let mut path = vec![];
        //note that the first and last element SHOULD be the same?
        path.push((y, x));
        for instruction in &self.data {
            match instruction.direction {
                0 => {
                    y -= instruction.distance;
                    min_y = min_y.min(y);
                }
                1 => {
                    x -= instruction.distance;
                    min_x = min_x.min(x);
                }
                2 => {
                    y += instruction.distance;
                    max_y = max_y.max(y);
                }
                3 => {
                    x += instruction.distance;
                    max_x = max_x.max(x);
                }
                _ => panic!("Invalid direction"),
            }
            path.push((y, x));
        }
        path.pop();
        let normal_path = path
            .into_iter()
            .map(|(y, x)| ((y - min_y) as usize, (x - min_x) as usize))
            .collect();
        (
            normal_path,
            ((max_y - min_y) as usize, (max_x - min_x) as usize),
        )
    }

    fn calculate_lagoon(&self, print: bool) -> usize {
        let mut total = 0;
        let (mut path, (y_max, x_max)) = self.make_path();
        //sort by y, so we can then hopefully just scan down rows
        //this will be in reverse order to make popping faster, could just dequeue but whatever
        path.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        let mut prev_line: VecDeque<(usize, usize)> = VecDeque::new();
        let mut ys = path
            .iter()
            .map(|(y, _)| vec![*y - 1, *y, *y + 1])
            .flatten()
            .collect::<HashSet<usize>>()
            .into_iter()
            .filter(|y| *y <= y_max)
            .collect::<Vec<usize>>();
        ys.sort();
        let fold_ranges = |line: &mut Vec<(usize, usize)>| {
            if line.len() <= 1 {
                return;
            }
            let mut i = 0;
            line.sort_by(|a, b| a.0.cmp(&b.0));
            while i < line.len() - 1 {
                if line[i].1 >= line[i + 1].0 {
                    line[i].1 = line[i].1.max(line[i + 1].1);
                    line.remove(i + 1);
                } else {
                    i += 1;
                }
            }
        };
        for i in 0..ys.len() {
            let yi = ys[i];
            let mut built_current_line = Vec::new();
            let mut current_line = VecDeque::new();
            if path.is_empty() {
                panic!("Path is empty, we might be just breaking here but idk");
            }
            let mut temp = vec![];
            while !path.is_empty() && path.last().unwrap().0 == yi {
                temp.push(path.pop().unwrap().1);
            }
            if temp.len() % 2 == 1 {
                panic!("temp is odd, this means there is no opposing edge to the lagoon");
            }
            temp.sort();
            for i in (0..temp.len()).step_by(2) {
                current_line.push_back((temp[i], temp[i + 1]));
                built_current_line.push((temp[i], temp[i + 1]));
            }
            let mut next_line = Vec::new();
            let mut prev_o = prev_line.pop_front();
            let mut current_o = current_line.pop_front();
            loop {
                match (prev_o, current_o) {
                    (None, None) => break,
                    (Some(prev), None) => {
                        next_line.push(prev);
                        built_current_line.push(prev);
                        prev_o = prev_line.pop_front();
                    }
                    (None, Some(current)) => {
                        next_line.push(current);
                        built_current_line.push(current);
                        current_o = current_line.pop_front();
                    }
                    (Some(prev), Some(current)) => {
                        if current.0 >= current.1 || prev.0 >= prev.1 {
                            panic!("current or prev is invalid, this should never happen");
                        }
                        //this use to be a match, but honestly it was more confusing lol
                        if current.0 > prev.1 {
                            //prev entirelly before
                            built_current_line.push(prev);
                            next_line.push(prev);
                            prev_o = prev_line.pop_front();
                        } else if current.1 < prev.0 {
                            //prev entirelly after
                            built_current_line.push(current);
                            next_line.push(current);
                            current_o = current_line.pop_front();
                        } else {
                            if current == prev {
                                built_current_line.push(prev);
                                prev_o = prev_line.pop_front();
                            } else if current.0 == prev.1 {
                                //expand right
                                let new = (prev.0, current.1);
                                built_current_line.push(new);
                                prev_o = Some(new);
                            } else if current.1 == prev.0 {
                                //expand left
                                let new = (current.0, prev.1);
                                built_current_line.push(new);
                                prev_o = Some(new);
                            } else if current.0 == prev.0 {
                                //reduce inner left
                                let new = (current.1, prev.1);
                                built_current_line.push(prev);
                                prev_o = Some(new);
                            } else if current.1 == prev.1 {
                                //reduce inner right
                                let new = (prev.0, current.0);
                                built_current_line.push(prev);
                                prev_o = Some(new);
                            } else if current.0 > prev.0 && current.1 < prev.1 {
                                built_current_line.push(prev);
                                let block_1 = (prev.0, current.0);
                                let block_2 = (current.1, prev.1);
                                next_line.push(block_1);
                                prev_o = Some(block_2);
                            } else {
                                panic!("This should never happen");
                            }
                            current_o = current_line.pop_front();
                            //overlap
                        }
                    }
                }
            }
            fold_ranges(&mut built_current_line);
            fold_ranges(&mut next_line);
            let line_total = built_current_line
                .iter()
                .fold(0, |acc, (a, b)| acc + b - a + 1);
            let mult = if i < ys.len() - 1 {
                ys[i + 1] - ys[i]
            } else {
                1
            };
            total += line_total * mult;
            prev_line = VecDeque::from(next_line);
            if print {
                let mut i = 0;
                println!();
                for x in 0..=x_max {
                    while i < built_current_line.len() && built_current_line[i].1 < x {
                        i += 1;
                    }
                    if i >= built_current_line.len() {
                        print!(".");
                        continue;
                    }
                    if x >= built_current_line[i].0 && x <= built_current_line[i].1 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                print!("\tlinetotal:{}, nbl:{:?}", line_total, built_current_line);
            }
        }
        if print {
            println!();
        }
        total
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    // data.calculate_lagoon(true).to_string()
    data.calculate_lagoon(false).to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import_pt2(lines);
    data.calculate_lagoon(false).to_string()
}
pub fn test_data() -> &'static str {
    "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
}

fn import(lines: &Vec<String>) -> Day18Data {
    Day18Data {
        data: lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut data = l.trim().split(' ');
                Instruction {
                    direction: match data.next().expect("No direction").chars().next().unwrap() {
                        'U' => 0,
                        'L' => 1,
                        'D' => 2,
                        'R' => 3,
                        _ => panic!("Invalid direction"),
                    },
                    distance: data
                        .next()
                        .expect("No distance")
                        .parse::<isize>()
                        .expect("Invalid distance"),
                }
            })
            .collect(),
    }
}
fn import_pt2(lines: &Vec<String>) -> Day18Data {
    Day18Data {
        data: lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let hex = l
                    .trim()
                    .split(' ')
                    .last()
                    .expect("No hex block")
                    .trim_start_matches("(#")
                    .trim_end_matches(")");
                let raw_direction = hex.bytes().last().expect("No direction digit");
                // I had ULDR, this one is RDLU so just reverse numbers
                let direction = (3 - (raw_direction - b'0')) % 4;
                Instruction {
                    direction,
                    distance: isize::from_str_radix(&hex[0..hex.len() - 1], 16)
                        .expect("Invalid RGB"),
                }
            })
            .collect(),
    }
}
