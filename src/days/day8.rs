use crate::util::Problem;
use num::integer::lcm;

pub const DAY8: Problem = Problem {
    day: 8,
    part1,
    part2,
    test_data: Some(test_data),
};
enum Direction {
    LEFT,
    RIGHT,
}

struct Day8Data {
    instructions: Vec<Direction>,
    //26^3 = 17576
    nodes: [Node; 17576],
    active: Vec<usize>,
}
#[derive(Clone, Copy)]
struct Node {
    left: usize,
    right: usize,
}
fn text_value(text: &str) -> usize {
    const A: u8 = b'A';
    text.bytes().enumerate().fold(0, |value, (i, c)| {
        value + ((c - A) as usize * 26_usize.pow(i as u32))
    })
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let goal = text_value("ZZZ");
    let mut i = text_value("AAA");
    let mut step = 0;
    while i != goal {
        for direction in &data.instructions {
            i = match direction {
                Direction::LEFT => data.nodes[i].left,
                Direction::RIGHT => data.nodes[i].right,
            };
        }
        step += 1;
    }
    format!("{}", step * data.instructions.len())
}

fn end_node(node: usize) -> bool {
    const GOAL: usize = (b'Z' - b'A') as usize * 26_usize.pow(2);
    node / GOAL == 1
}
fn start_node(node: usize) -> bool {
    const GOAL: usize = 26_usize.pow(2);
    node < GOAL
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let starts: Vec<usize> = data.active.clone();
    let mut initial: Vec<usize> = vec![0; starts.len()];
    // let mut loop_len: Vec<usize> = vec![0; starts.len()];
    for i in 0..starts.len() {
        let mut node = starts[i];
        let mut step = 0;
        while !end_node(node) {
            for direction in &data.instructions {
                node = match direction {
                    Direction::LEFT => data.nodes[node].left,
                    Direction::RIGHT => data.nodes[node].right,
                };
            }
            step += 1;
        }
        initial[i] = step;
        // step = 0;
        // for direction in &data.instructions {
        //     node = match direction {
        //         Direction::LEFT => data.nodes[node].left,
        //         Direction::RIGHT => data.nodes[node].right,
        //     };
        // }
        // step += 1;
        // println!("next step {}", node);
        // while !end_node(node) {
        //     for direction in &data.instructions {
        //         node = match direction {
        //             Direction::LEFT => data.nodes[node].left,
        //             Direction::RIGHT => data.nodes[node].right,
        //         };
        //     }
        //     step += 1;
        // }
        // loop_len[i] = step;
    }
    //oh my god... initial=loop_len
    //and because of this we can just least common multiple them because the loops aren't offset by
    //the initial....
    //
    // println!("Initial: {:?}", initial);
    // println!("Loop len: {:?}", loop_len);
    // let mut steps = initial.clone();
    // let mut min_index = 0;
    // let mut min = usize::MAX;
    // while steps.iter().any(|&x| x != steps[0]) {
    //     for i in 0..steps.len() {
    //         if steps[i] < min {
    //             min = steps[i];
    //             min_index = i;
    //         }
    //     }
    //     steps[min_index] += loop_len[min_index];
    //     min = usize::MAX;
    //     println!("{:?}", steps);
    // }
    let lcm = initial.iter().fold(1, |acc, &x| lcm(acc, x));
    format!("{:?}", lcm * data.instructions.len())
}
pub fn test_data() -> String {
    "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
        .to_owned()
}

fn import(lines: &Vec<String>) -> Day8Data {
    // let mut nodes = vec![Node { left: 0, right: 0 }; 456976];
    let mut nodes = [Node { left: 0, right: 0 }; 17576];
    let instructions = lines[0]
        .trim()
        .bytes()
        .map(|c| {
            if c == b'L' {
                Direction::LEFT
            } else {
                Direction::RIGHT
            }
        })
        .collect();
    let remove = [
        lines[2].find(')').unwrap(),
        lines[2].find(',').unwrap(),
        lines[2].find('(').unwrap(),
        lines[2].find('=').unwrap(),
    ];
    let mut active = Vec::new();
    lines[2..]
        .iter()
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let mut line = line.clone();
            for i in remove.iter() {
                line.remove(*i);
            }
            let mut parts = line.split_whitespace();
            let node_text = parts.next().unwrap();
            let node = text_value(node_text);
            if start_node(node) {
                active.push(node);
            }
            nodes[node].left = text_value(parts.next().unwrap());
            nodes[node].right = text_value(parts.next().unwrap());
        });

    Day8Data {
        instructions,
        nodes,
        active,
    }
}
//tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_startnodes() {
        let correct = vec!["AAA", "ABA", "AZA", "ZZA"];
        let wrong = vec!["AAJ", "ABB", "AZZ", "ZAZ"];
        for node in correct {
            assert!(start_node(text_value(node)));
        }
        for node in wrong {
            assert!(!start_node(text_value(node)));
        }
    }
    #[test]
    fn part2_endnodes() {
        let correct = vec!["AAZ", "JJZ", "ZZZ"];
        let wrong = vec!["BAA", "CBA", "ZZX", "ZZA"];
        for node in correct {
            assert!(
                end_node(text_value(node)),
                "Correct failure: node:{}, value:{}",
                node,
                text_value(node)
            );
        }
        for node in wrong {
            assert!(
                !end_node(text_value(node)),
                "Wrong failure: node:{}, value:{}",
                node,
                text_value(node)
            );
        }
    }
}
