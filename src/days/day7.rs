use crate::util::Problem;

const VALUES: [u8; 13] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
];
const JOKER: u8 = b'J';
const PT2_VALUES: [u8; 13] = [
    JOKER, b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
];
#[derive(Copy, Clone, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
impl Type {
    fn index(&self) -> usize {
        *self as usize
    }
}

struct Hand {
    bid: usize,
    value: usize,
}
impl Hand {
    fn new(contents: String, bid: usize, pt2: bool) -> Hand {
        let contents = contents.into_bytes();
        let mut freq = [0_u8; 13];
        let mut value = 0;
        contents.iter().rev().enumerate().for_each(|(i, x)| {
            let values = if pt2 { PT2_VALUES } else { VALUES };
            let index: usize = values.iter().position(|y| y == x).unwrap() as usize;
            if !pt2 || *x != JOKER {
                freq[index as usize] += 1;
            }
            //these could/should have been 13.pow but i started with 13^i and obviously that didn't
            //work lol
            value += index as usize * 100_usize.pow(i as u32);
        });
        let mut freq: Vec<u8> = freq.into_iter().filter(|x| *x > 0).collect();
        freq.sort();
        let hand_type = match freq.len() {
            0 | 1 => Type::FiveKind,
            2 => {
                if freq[0] == 1 {
                    Type::FourKind
                } else {
                    Type::FullHouse
                }
            }
            3 => {
                if freq[1] == 1 {
                    Type::ThreeKind
                } else {
                    Type::TwoPair
                }
            }
            4 => Type::OnePair,
            _ => Type::HighCard,
        };
        value += hand_type.index() * 1_000_000_000_000;
        Hand { bid, value }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Hand {}

pub const DAY7: Problem = Problem {
    day: 7,
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &Vec<String>) -> String {
    let mut hands = import(lines, false);
    hands.sort();
    let result: usize = hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.bid * (i + 1));
    format!("{}", result)
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut hands = import(lines, true);
    hands.sort();
    let result: usize = hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.bid * (i + 1));
    format!("{}", result)
}
pub fn test_data() -> String {
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_owned()
}

fn import(lines: &Vec<String>, pt2: bool) -> Vec<Hand> {
    //probably not needed half the time
    lines
        .iter()
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                let mut split = x.split_whitespace();
                let hand = split.next().unwrap().to_owned();
                let bid = split
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Unable to parse bet");
                Some(Hand::new(hand, bid, pt2))
            }
        })
        .collect()
}
