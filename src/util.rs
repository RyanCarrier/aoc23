use std::{fs, path::Path, time::Duration};
#[allow(dead_code)]
pub static TRANSFORMS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

pub fn format_duration(d: Duration) -> String {
    if d.as_millis() > 1000 {
        return format!("{:.1}s", d.as_millis() as f64 / 1000 as f64);
    }
    if d.as_micros() > 1000 {
        return format!("{:.1}ms", d.as_micros() as f64 / 1000 as f64);
    }
    if d.as_nanos() > 1000 {
        return format!("{:.1}μs", d.as_nanos() as f64 / 1000 as f64);
    }
    format!("{}ns", d.as_nanos())
}

pub fn get_input_data(year: usize, day: usize) -> Vec<String> {
    // get input data from aoc using cookie
    if !Path::new("./cookie.txt").exists() {
        panic!("cookie in a file called cookie.txt plzx");
    }
    let cookie = fs::read_to_string("./cookie.txt").unwrap();
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .unwrap();
    let body = resp.text().unwrap();
    body.split('\n').map(|x| x.to_owned()).collect()
}

pub struct Problem {
    pub day: usize,
    pub part1: fn(input: Vec<String>) -> String,
    pub part2: fn(input: Vec<String>) -> String,
    pub test_data: fn() -> Option<Vec<String>>,
}
