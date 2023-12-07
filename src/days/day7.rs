use crate::util::Problem;

const VALUES: [u8; 13] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
];
enum Type {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}
struct Hand {
    contents: [u8; 5],
    hand_type: Type,
    value: usize,
}
impl Hand {
    fn new(contents: String) -> Hand {
        //TODO: calculate tings
        Hand {
            contents,
            hand_type: Type::HighCard,
            value: 0,
        }
    }
}

pub const DAY7: Problem = Problem {
    day: 7,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day7Data {
    data: Vec<i32>,
}

pub fn part1(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}

pub fn part2(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}
pub fn test_data() -> String {
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_owned()
}

fn import(lines: &Vec<String>) -> Day7Data {
    //probably not needed half the time
    Day7Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
