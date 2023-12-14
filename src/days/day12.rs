use std::{collections::HashMap, fmt::Debug};

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
    memo: HashMap<Spring, usize>,
}
impl Day12Data {
    fn new(springs: Vec<Spring>) -> Self {
        Day12Data {
            springs,
            memo: HashMap::new(),
        }
    }
    fn process2(&mut self) -> Vec<usize> {
        self.springs
            .clone()
            .iter_mut()
            .map(|s| s.handle2(0, 0))
            .collect()
    }
    //could go memoized or try do something with nPr
    fn process(&mut self) -> Vec<usize> {
        self.springs
            .clone()
            .iter()
            .map(|s| self.handle(s.clone()))
            .collect()
    }
    //could go memoized or try do something with nPr
    fn handle(&mut self, original: Spring) -> usize {
        let mut spring = original.clone();
        if spring.trim() {
            return self.handle(spring);
        }
        if self.memo.contains_key(&spring) {
            return self.memo[&spring];
        }
        if spring.damaged.len() == 0 {
            if spring.records.iter().any(|&x| x == DAMAGED) {
                return 0;
            } else {
                return 1;
            }
        }
        if spring.records.len() == 0 {
            return 0;
        }
        if spring.damaged.len() - 1 + spring.damaged.iter().sum::<usize>() > spring.records.len() {
            return 0;
        }
        let result = match spring.records[0] {
            UNKNOWN => {
                self.handle(spring.clone_unknown(false)) + self.handle(spring.clone_unknown(true))
            }
            DAMAGED => {
                if spring.can_cut() {
                    self.handle(spring.clone_cut())
                } else {
                    0
                }
            }
            _ => panic!("Unknown record type, (parsing error)"),
        };
        self.memo.insert(spring, result);
        result
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
    fn clone_unknown(&self, damaged: bool) -> Self {
        let mut records = self.records.clone();
        records[0] = if damaged { DAMAGED } else { OPERATIONAL };
        Spring {
            records,
            damaged: self.damaged.clone(),
            memo: self.memo.clone(),
        }
    }
    fn can_cut(&self) -> bool {
        let can_fit_damaged = self.damaged[0] <= self.records.len()
            && self.records[0..self.damaged[0]]
                .iter()
                .all(|x| *x != OPERATIONAL);
        //verify we are at the end of the record OR the next token can be used as a
        //seperator (either damaged or unknown)
        let next_token_ok =
            self.damaged[0] == self.records.len() || self.records[self.damaged[0]] != DAMAGED;
        can_fit_damaged && next_token_ok
    }
    fn can_cut2(&self, index: usize, damaged_index: usize) -> bool {
        let can_fit_damaged = self.damaged[damaged_index] <= self.records[index..].len()
            && self.records[index..index + self.damaged[damaged_index]]
                .iter()
                .all(|x| *x != OPERATIONAL);
        //verify we are at the end of the record OR the next token can be used as a
        //seperator (either damaged or unknown)
        let next_token_ok = index + self.damaged[damaged_index] == self.records.len()
            || self.records[index + self.damaged[damaged_index]] != DAMAGED;
        can_fit_damaged && next_token_ok
    }

    fn clone_cut(&self) -> Self {
        if self.records.len() == self.damaged[0] {
            return Spring {
                records: vec![],
                damaged: self.damaged[1..].to_vec(),
                memo: vec![],
            };
        }
        let index_from = self.damaged[0] + 1;
        Spring {
            //becuase we must also cut the .
            records: self.records[index_from..].to_vec(),
            damaged: self.damaged[1..].to_vec(),
            memo: vec![],
        }
    }
    fn trim(&mut self) -> bool {
        let mut result = false;
        while self.records.len() > 0 && self.records[0] == OPERATIONAL {
            self.records.remove(0);
            result = true;
        }
        while self.records.len() > 0 && self.records[self.records.len() - 1] == OPERATIONAL {
            self.records.pop();
            result = true;
        }
        result
    }
    fn handle2(&mut self, index: usize, damaged_index: usize) -> usize {
        if index >= self.records.len() {
            return 0;
        }
        if self.damaged.len() == damaged_index {
            if self.records[index..].iter().any(|&x| x == DAMAGED) {
                return 0;
            } else {
                return 1;
            }
        }
        if self.damaged.len() - damaged_index - 1
            + self.damaged[damaged_index..].iter().sum::<usize>()
            > self.records[index..].len()
        {
            return 0;
        }
        if let Some(result) = self.memo[index][damaged_index] {
            return result;
        }
        if self.records[index] == OPERATIONAL {
            return self.handle2(index + 1, damaged_index);
        }
        let mut result = 0;
        if self.records[index] == UNKNOWN {
            result += self.handle2(index + 1, damaged_index);
        }
        if self.can_cut2(index, damaged_index) {
            // println!("cutting {:?} {:?}", index, damaged_index);
            result += self.handle2(index + self.damaged[damaged_index], damaged_index + 1);
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
    //BROKEN LOL
    format!("{:?}", data.process2().iter().sum::<usize>())
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
