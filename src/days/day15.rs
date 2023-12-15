use crate::util::Problem;

pub const DAY15: Problem = Problem {
    day: 15,
    part1,
    part2,
    test_data: Some(test_data),
};

fn hash(s: &str) -> usize {
    let mut current_value = 0_usize;
    for c in s.bytes() {
        current_value = (current_value + c as usize) * 17 % 256;
    }
    current_value
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    data.fold(0, |acc, s| acc + hash(s)).to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for d in data {
        let mut hash = 0;
        let mut splitter = '-';
        let mut bytes = d.bytes();
        while let Some(c) = bytes.next() {
            match c {
                b'=' => {
                    splitter = '=';
                    break;
                }
                b'-' => break,
                _ => hash = (hash + c as usize) * 17 % 256,
            }
        }
        let label = d.split_once(splitter).unwrap().0;
        let index = boxes[hash].iter().position(|x| x.0 == label);
        if splitter == '=' {
            let n = (bytes.next().unwrap() - b'0') as usize;
            if let Some(i) = index {
                boxes[hash][i].1 = n;
            } else {
                boxes[hash].push((label, n));
            }
        } else if let Some(i) = index {
            boxes[hash].remove(i);
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, v)| {
            acc + v
                .iter()
                .enumerate()
                .fold(0, |sub_acc, x| sub_acc + (i + 1) * (x.0 + 1) * (x.1 .1))
        })
        .to_string()
}
pub fn test_data() -> &'static str {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
}

fn import(lines: &Vec<String>) -> impl Iterator<Item = &str> {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .next()
        .unwrap()
        .trim()
        .split(',')
}
