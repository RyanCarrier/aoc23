use crate::util::Problem;

pub const DAY6: Problem = Problem {
    day: 6,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day6Data {
    races: Vec<Race>,
}
struct Race {
    duration: usize,
    distance: usize,
}
impl Race {
    fn min_charge_times(&self) -> (usize, usize) {
        //find roots of distance=charge(duration-charge)
        //ie; 0=-charge^2+duration*charge-distance
        let discriminant: isize =
            (self.duration * self.duration) as isize - 4 * (self.distance + 1) as isize;
        //+1 to distance because we need to BEAT the distance and time, not match it
        if discriminant < 0 {
            panic!("Negative discriminant");
        }
        let half_root = (discriminant as f64).sqrt() / 2.0;
        let half_t = self.duration as f64 / 2.0;
        let (mut b, mut a) = ((half_t + half_root).floor(), (half_t - half_root).ceil());
        //because i just want a to be smaller
        if a.is_sign_negative() {
            //this might actually never happen with these cases
            a = b;
        } else if b.is_sign_negative() {
            b = a;
        }
        //we may need to step up or down these but we will see how we go
        (a as usize, b as usize)
    }
    fn all_winning_times(&self) -> Vec<usize> {
        let (a, b) = self.min_charge_times();
        (a..=b).collect()
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let product_of_len: usize = data
        .races
        .iter()
        .map(|x| x.all_winning_times().len())
        .product::<usize>();
    format!("{}", product_of_len)
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import_pt2(lines);
    let ways_to_win = data.races[0].all_winning_times().len();
    format!("{}", ways_to_win)
}
pub fn test_data() -> String {
    "Time:      7  15   30
Distance:  9  40  200"
        .to_owned()
}
fn import_pt2(lines: &Vec<String>) -> Day6Data {
    //probably not needed half the time
    let duration = lines[0]
        .split(":")
        .last()
        .expect("Duration (1st) line should have :")
        .trim()
        .replace(" ", "")
        .parse::<usize>()
        .expect("Duration should be a number");
    let distance = lines[1]
        .split(":")
        .last()
        .expect("Distance (2nd) line should have :")
        .trim()
        .replace(" ", "")
        .parse::<usize>()
        .expect("Duration should be a number");
    Day6Data {
        races: vec![Race { duration, distance }],
    }
}

fn import(lines: &Vec<String>) -> Day6Data {
    //probably not needed half the time
    let duration = lines[0]
        .split(":")
        .last()
        .expect("Duration (1st) line should have :")
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let distance = lines[1]
        .split(":")
        .last()
        .expect("Distance (2nd) line should have :")
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let races = duration
        .zip(distance)
        .map(|(duration, distance)| Race { duration, distance })
        .collect();
    Day6Data { races }
}
