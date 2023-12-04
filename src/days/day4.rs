use crate::util::Problem;

pub const DAY4: Problem = Problem {
    day: 4,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Game {
    winning: Vec<i32>,
    our: Vec<i32>,
}
impl Game {
    fn matches(&self) -> Vec<i32> {
        self.winning
            .iter()
            .filter(|x| self.our.contains(x))
            .map(|x| *x)
            .collect()
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let games = import(lines);
    let mut total = 0;
    games.iter().for_each(|g| {
        let matches = g.matches();
        if matches.is_empty() {
            return;
        }
        total += 2_i32.pow((matches.len() - 1) as u32);
    });
    format!("{}", total)
}

pub fn part2(lines: &Vec<String>) -> String {
    let games = import(lines);
    let mut cards = vec![1; games.len()];
    games.iter().enumerate().for_each(|(i, g)| {
        let matches = g.matches();
        if matches.is_empty() {
            return;
        }
        for j in 0..matches.len() {
            cards[i + j + 1] += cards[i];
        }
    });
    // println!("{:?}", cards);
    format!("{}", cards.into_iter().sum::<usize>())
}
pub fn test_data() -> String {
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_owned()
}

fn import(lines: &Vec<String>) -> Vec<Game> {
    //probably not needed half the time
    lines
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut number_sets = line
                .split(":")
                .last()
                .expect("Should have :")
                .trim()
                .split("|");
            // println!("{:?}", number_sets.clone().collect::<Vec<&str>>());
            let winning_numbers = number_sets
                .next()
                .expect("Should have winning numbers")
                .trim()
                .split(" ")
                .filter_map(|x| {
                    if x.is_empty() {
                        return None;
                    }
                    Some(x.parse::<i32>().expect("Winning splits should be a number"))
                })
                .collect::<Vec<i32>>();
            let our_numbers = number_sets
                .next()
                .expect("Should have our numbers")
                .trim()
                .split(" ")
                .filter_map(|x| {
                    if x.is_empty() {
                        return None;
                    }
                    Some(x.parse::<i32>().expect("Our splits should be a number"))
                })
                .collect::<Vec<i32>>();
            Some(Game {
                winning: winning_numbers,
                our: our_numbers,
            })
        })
        .collect()
}
