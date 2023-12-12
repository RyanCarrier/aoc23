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
    data: Vec<Spring>,
}
struct Spring {
    records: Vec<u8>,
    damaged: Vec<usize>,
}
impl Spring {
    fn new(line: &String) -> Self {
        let mut split = line.trim().split_whitespace();
        Spring {
            records: split.next().unwrap().as_bytes().to_vec(),
            damaged: split
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
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
        data: lines
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(Spring::new)
            .collect(),
    }
}
