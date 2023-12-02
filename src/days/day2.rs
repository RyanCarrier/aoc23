use crate::util::Problem;

pub const DAY2: Problem = Problem {
    day: 2,
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &Vec<String>) -> String {
    const RED: usize = 12;
    const GREEN: usize = 13;
    const BLUE: usize = 14;
    let mut id_sum = 0;
    for l in lines {
        if l.is_empty() {
            continue;
        }
        let mut gamesplit = l.split(":");
        let game_id = gamesplit
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for g in gamesplit.last().unwrap().split(";") {
            let game_split = g.split(",");
            for ball in game_split {
                let ball_data = ball.trim().split(" ").collect::<Vec<&str>>();
                let ball_amount = ball_data[0].parse::<usize>().unwrap();
                let ball_colour = ball_data.last().unwrap().trim();
                match ball_colour {
                    "red" => red = red.max(ball_amount),
                    "green" => green = green.max(ball_amount),
                    "blue" => blue = blue.max(ball_amount),
                    _ => panic!("Unknown colour {}", ball_colour),
                }
            }
        }
        if red <= RED && green <= GREEN && blue <= BLUE {
            id_sum += game_id;
        }
    }
    id_sum.to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut id_sum = 0;
    for l in lines {
        if l.is_empty() {
            continue;
        }
        let gamesplit = l.split(":");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for g in gamesplit.last().unwrap().split(";") {
            let game_split = g.split(",");
            for ball in game_split {
                let ball_data = ball.trim().split(" ").collect::<Vec<&str>>();
                let ball_amount = ball_data[0].parse::<usize>().unwrap();
                let ball_colour = ball_data.last().unwrap().trim();
                match ball_colour {
                    "red" => red = red.max(ball_amount),
                    "green" => green = green.max(ball_amount),
                    "blue" => blue = blue.max(ball_amount),
                    _ => panic!("Unknown colour {}", ball_colour),
                }
            }
        }
        id_sum += red * green * blue;
    }
    id_sum.to_string()
}
pub fn test_data() -> String {
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_owned()
}
