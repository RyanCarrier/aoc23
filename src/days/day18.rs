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
    distance: usize,
    #[allow(dead_code)]
    rgb: u32,
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
                0 => y -= instruction.distance as isize,
                1 => x -= instruction.distance as isize,
                2 => y += instruction.distance as isize,
                3 => x += instruction.distance as isize,
                _ => panic!("Invalid direction"),
            }
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
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
        for i in 0..ys.len() {
            let yi = ys[i];
            let mut built_current_line = vec![];
            let mut current_line = VecDeque::new();
            if path.is_empty() {
                panic!("Path is empty, we might be just breaking here but idk");
            }
            let mut temp = vec![];
            while !path.is_empty() && path.last().unwrap().0 == yi {
                temp.push(path.pop().unwrap().1);
            }
            temp.sort();
            if temp.len() % 2 == 1 {
                panic!("temp is odd, this means there is no opposing edge to the lagoon");
            }
            for i in (0..temp.len()).step_by(2) {
                current_line.push_back((temp[i], temp[i + 1]));
                built_current_line.push((temp[i], temp[i + 1]));
            }

            let mut next_line = VecDeque::new();
            let mut prev_o = prev_line.pop_front();
            let mut current_o = current_line.pop_front();
            let add_to_next = |next_line: &mut VecDeque<(usize, usize)>, item: (usize, usize)| {
                if next_line.is_empty() {
                    next_line.push_back(item);
                } else {
                    let li = next_line.len() - 1;
                    let nl = next_line[li];
                    if nl.0 == item.0 {
                        if nl.1 < item.1 {
                            next_line[li].1 = item.1;
                        }
                    } else {
                        if nl.1 == item.0 {
                            next_line[li].1 = item.1;
                        } else {
                            next_line.push_back(item);
                        }
                    }
                }
                // print!("\tnl:{:?}", next_line);
            };
            loop {
                match (prev_o, current_o) {
                    (None, None) => break,
                    (Some(prev), None) => {
                        add_to_next(&mut next_line, prev);
                        built_current_line.push(prev);
                        prev_o = prev_line.pop_front();
                    }
                    (None, Some(current)) => {
                        add_to_next(&mut next_line, current);
                        built_current_line.push(current);
                        current_o = current_line.pop_front();
                    }
                    (Some(prev), Some(current)) => {
                        if current.0 >= current.1 || prev.0 >= prev.1 {
                            println!("current: {:?}, prev: {:?}", current, prev);
                            panic!("current or prev is invalid, this should never happen");
                        }
                        if current.0 > prev.1 {
                            //prev entirelly before
                            built_current_line.push(prev);
                            add_to_next(&mut next_line, prev);
                            prev_o = prev_line.pop_front();
                        } else if current.1 < prev.0 {
                            //prev entirelly after
                            built_current_line.push(current);
                            add_to_next(&mut next_line, current);
                            current_o = current_line.pop_front();
                        } else {
                            if current == prev {
                                built_current_line.push(prev);
                                current_o = current_line.pop_front();
                                prev_o = prev_line.pop_front();
                            } else if current.0 == prev.1 {
                                //expand right
                                let new = (prev.0, current.1);
                                built_current_line.push(new);
                                if new.0 >= new.1 {
                                    panic!("new is invalid, this should never happen 0==1");
                                }
                                prev_o = Some(new);
                                current_o = current_line.pop_front();
                            } else if current.1 == prev.0 {
                                //expand left
                                let new = (current.0, prev.1);
                                built_current_line.push(new);
                                if new.0 >= new.1 {
                                    panic!("new is invalid, this should never happen 1==0");
                                }
                                prev_o = Some(new);
                                current_o = current_line.pop_front();
                            } else if current.0 == prev.0 {
                                //reduce inner left
                                let new = (current.1, prev.1);
                                built_current_line.push(prev);
                                current_o = current_line.pop_front();
                                if new.0 >= new.1 {
                                    panic!("new is invalid, this should never happen 0==0");
                                }
                                prev_o = Some(new);
                            } else if current.1 == prev.1 {
                                //reduce inner right
                                let new = (prev.0, current.0);
                                built_current_line.push(prev);
                                if new.0 >= new.1 {
                                    panic!("new is invalid, this should never happen 1==1");
                                }
                                prev_o = Some(new);
                                current_o = current_line.pop_front();
                                // println!("current: {:?}, prev: {:?}", current, prev);
                            } else if current.0 > prev.0 && current.1 < prev.1 {
                                built_current_line.push(prev);
                                let block_1 = (prev.0, current.0);
                                let block_2 = (current.1, prev.1);
                                add_to_next(&mut next_line, block_1);
                                prev_o = Some(block_2);
                                current_o = current_line.pop_front();
                            } else {
                                panic!("This should never happen");
                            }
                            //overlap
                        }
                    }
                }
            }
            //fold built current line
            built_current_line.sort_by(|a, b| a.0.cmp(&b.0).reverse());
            let mut new_built_line = vec![];
            let mut trench = built_current_line.pop().unwrap();
            while let Some(current) = built_current_line.pop() {
                if trench.1 < current.0 {
                    new_built_line.push(trench);
                    trench = current.clone();
                } else {
                    trench.1 = trench.1.max(current.1);
                }
            }
            new_built_line.push(trench);
            let line_total = new_built_line.iter().fold(0, |acc, (a, b)| acc + b - a + 1);
            let mult = if i < ys.len() - 1 {
                ys[i + 1] - ys[i]
            } else {
                1
            };
            total += line_total * mult;
            prev_line = next_line;
            if print {
                let mut i = 0;
                println!();
                for x in 0..=x_max {
                    while i < new_built_line.len() && new_built_line[i].1 < x {
                        i += 1;
                    }
                    if i >= new_built_line.len() {
                        print!(".");
                        continue;
                    }
                    if x >= new_built_line[i].0 && x <= new_built_line[i].1 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        total
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    //3242 too low/
    //18446744073709538691 too high
    //63623 too high
    //47722 incorrect
    //42276 incorrect
    data.calculate_lagoon(false).to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let _ = import(lines);
    "".to_owned()
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
                        .parse::<usize>()
                        .expect("Invalid distance"),
                    rgb: u32::from_str_radix(
                        data.next()
                            .expect("No RGB")
                            .trim_start_matches("(#")
                            .trim_end_matches(")"),
                        16,
                    )
                    .expect("Invalid RGB"),
                }
            })
            .collect(),
    }
}
