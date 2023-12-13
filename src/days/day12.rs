use std::collections::HashMap;

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
    fn memo_return(&mut self, spring: Spring, result: usize) -> usize {
        self.memo.insert(spring, result);
        result
    }
    fn process(&mut self) -> Vec<usize> {
        self.springs
            .clone()
            .iter()
            .map(|s| self.handle(s))
            .collect()
    }
    //could go memoized or try do something with nPr
    fn handle(&mut self, spring: &Spring) -> usize {
        let mut spring = spring.clone();
        spring.trim();
        if self.memo.contains_key(&spring) {
            return self.memo[&spring];
        }
        if spring.records.len() == 0 || spring.damaged.len() == 0 {
            return self.memo_return(spring, 0);
        }
        if spring.damaged.len() - 1 + spring.damaged.iter().sum::<usize>() > spring.records.len() {
            return self.memo_return(spring, 0);
        }
        println!("{:?}", spring);
        return match spring.records[0] {
            UNKNOWN => {
                let mut new_spring = spring.clone();
                let mut result = 0;
                new_spring.records[0] = OPERATIONAL;
                result += self.handle(&new_spring);
                new_spring.records[0] = DAMAGED;
                result += self.handle(&new_spring);
                new_spring.records[0] = UNKNOWN;
                self.memo_return(spring, result)
            }
            DAMAGED => {
                //if we have space and we can fit the damaged block
                if spring.damaged[0] < spring.records.len()
                    && spring.records[0..spring.damaged[0]]
                        .iter()
                        .all(|x| *x != OPERATIONAL)
                {
                    spring.records = spring.records[spring.damaged[0]..].to_vec();
                    spring.damaged.remove(0);
                    let mut result = 1;
                    result += self.handle(&spring);
                    return self.memo_return(spring, result);
                }
                self.memo_return(spring, 0)
            }
            OPERATIONAL => {
                panic!("Operational should have been removed prior to here");
            }
            _ => panic!("Unknown record type, (parsing error)"),
        };
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Spring {
    records: Vec<u8>,
    damaged: Vec<usize>,
}

impl Spring {
    fn new(line: &String) -> Self {
        let mut split = line.trim().split_whitespace();
        Spring {
            records: split.next().unwrap().replace("..", ".").as_bytes().to_vec(),
            damaged: split
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }
    fn trim(&mut self) {
        while self.records.len() > 0 && self.records[0] == OPERATIONAL {
            self.records.remove(0);
        }
        while self.records.len() > 0 && self.records[self.records.len() - 1] == OPERATIONAL {
            self.records.pop();
        }
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    format!("{:?}", data.process())
}

pub fn part2(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
}

fn import(lines: &Vec<String>) -> Day12Data {
    //probably not needed half the time
    Day12Data {
        memo: HashMap::new(),
        springs: lines
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(Spring::new)
            .collect(),
    }
}
