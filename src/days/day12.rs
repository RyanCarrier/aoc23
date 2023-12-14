use std::fmt::Debug;

use crate::util::Problem;

pub const DAY12: Problem = Problem {
    day: 12,
    part1,
    part2,
    test_data: Some(test_data),
};
const UNKNOWN: u8 = b'?';
const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
struct Day12Data {
    springs: Vec<Spring>,
}
impl Day12Data {
    fn new(springs: Vec<Spring>) -> Self {
        Day12Data { springs }
    }
    fn process(&mut self) -> Vec<usize> {
        self.springs
            .clone()
            .iter_mut()
            .map(|s| s.handle(0, 0))
            .collect()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
struct Spring {
    records: Vec<u8>,
    damaged: Vec<usize>,
    memo: Vec<Vec<Option<usize>>>,
}
impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}",
            String::from_utf8(self.records.clone()).unwrap(),
            self.damaged
        )
    }
}

impl Spring {
    fn new(line: &String) -> Self {
        let mut split = line.trim().split_whitespace();
        let mut records = split.next().unwrap().trim().to_owned();
        while records.contains("..") {
            records = records.replace("..", ".");
        }
        let damaged: Vec<usize> = split
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let memo = vec![vec![None; damaged.len()]; records.len()];
        Spring {
            records: records.into_bytes(),
            damaged,
            memo,
        }
    }
    fn can_cut(&self, index: usize, damaged_index: usize) -> Option<usize> {
        let next_index = index + self.damaged[damaged_index];
        if next_index > self.records.len()
            || self.records[index..next_index]
                .iter()
                .any(|x| *x == OPERATIONAL)
        {
            return None;
        }
        //verify we are at the end of the record OR the next token can be used as a
        //seperator (either damaged or unknown)
        if next_index == self.records.len() {
            return Some(next_index);
        }
        if self.records[next_index] == DAMAGED {
            return None;
        }
        return Some(next_index + 1);
    }

    fn trim(&mut self) {
        while self.records.len() > 0 && self.records[0] == OPERATIONAL {
            self.records.remove(0);
        }
        while self.records.len() > 0 && self.records[self.records.len() - 1] == OPERATIONAL {
            self.records.pop();
        }
    }
    fn handle(&mut self, index: usize, damaged_index: usize) -> usize {
        if self.damaged.len() == damaged_index {
            if index == self.records.len() || self.records[index..].iter().all(|&x| x != DAMAGED) {
                return 1;
            } else {
                return 0;
            }
        }
        if index >= self.records.len() {
            return 0;
        }
        if let Some(result) = self.memo[index][damaged_index] {
            return result;
        }
        if self.damaged.len() - damaged_index - 1
            + self.damaged[damaged_index..].iter().sum::<usize>()
            > self.records[index..].len()
        {
            self.memo[index][damaged_index] = Some(0);
            return 0;
        }
        let mut result = 0;
        if self.records[index] == OPERATIONAL {
            result += self.handle(index + 1, damaged_index);
        } else {
            if self.records[index] == UNKNOWN {
                result += self.handle(index + 1, damaged_index);
            }
            if let Some(next) = self.can_cut(index, damaged_index) {
                result += self.handle(next, damaged_index + 1);
            }
        }
        self.memo[index][damaged_index] = Some(result);
        result
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    format!("{:?}", data.process().iter().sum::<usize>())
}

pub fn part2(lines: &Vec<String>) -> String {
    let new_lines = part2_input(lines);
    let mut data = import(&new_lines);
    data.springs.iter_mut().for_each(|s| s.trim());

    //BROKEN LOL
    format!("{:?}", data.process().iter().sum::<usize>())
}
pub fn test_data() -> &'static str {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
}

fn part2_input(lines: &Vec<String>) -> Vec<String> {
    lines
        .clone()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut split = l.split_whitespace();
            let mut records = split.next().unwrap().to_owned();
            let mut damaged = split.next().unwrap().to_owned();
            records.push_str("?");
            records = records.repeat(5);
            records.pop();
            damaged.push_str(",");
            damaged = damaged.repeat(5);
            damaged.pop();
            format!("{} {}", records, damaged)
        })
        .collect()
}
fn import(lines: &Vec<String>) -> Day12Data {
    Day12Data::new(
        lines
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(Spring::new)
            .collect(),
    )
}
//tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let inputs = [
            "#?? 1".to_owned(),
            "????????#???#.#?? 11,1".to_owned(),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".to_owned(),
            ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".to_owned(),
        ]
        .to_vec();
        let expected = [1, 1, 1, 16384];
        let mut data = import(&inputs);
        let results = data.process();
        for i in 0..data.springs.len() {
            println!("{:?}: {}", data.springs[i], results[i]);
            assert_eq!(results[i], expected[i]);
        }
    }
}
