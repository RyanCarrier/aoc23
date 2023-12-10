use std::collections::VecDeque;

use crate::util::Problem;

const PIPES: [u8; 6] = [b'|', b'-', b'L', b'J', b'7', b'F'];
const PIPES_INSTANCE: [[bool; 4]; 6] = [
    [true, false, true, false],
    [false, true, false, true],
    [true, true, false, false],
    [true, false, false, true],
    [false, false, true, true],
    [false, true, true, false],
];

#[derive(PartialEq)]
struct Pipe {
    entries: [bool; 4],
    distance: usize,
}

#[derive(PartialEq)]
enum Piece {
    Pipe(Pipe),
    Empty,
    Start,
}
impl Piece {
    fn new(input: u8) -> Piece {
        match input {
            b'.' => Piece::Empty,
            b'S' => Piece::Start,
            _ => Piece::Pipe(Pipe::new(input)),
        }
    }
    fn is_pipe(&self) -> bool {
        match self {
            Piece::Pipe(_) => true,
            _ => false,
        }
    }
    fn pipe_open(&self, direction: usize) -> bool {
        match self {
            Piece::Pipe(pipe) => pipe.entries[direction],
            _ => false,
        }
    }
}

impl Pipe {
    fn new(input: u8) -> Pipe {
        Pipe {
            entries: PIPES_INSTANCE[PIPES.iter().position(|&x| x == input).unwrap()],
            distance: usize::MAX,
        }
    }
}

pub const DAY10: Problem = Problem {
    day: 10,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day10Data {
    data: Vec<Vec<Piece>>,
}
impl Day10Data {
    fn furthest_distance(&mut self, start: (usize, usize)) -> usize {
        let mut visited = vec![vec![false; self.data[0].len()]; self.data.len()];
        let mut queue = VecDeque::new();
        // queue.push_back((start, 0));
        let in_range = |x: usize, y: usize| x < self.data.len() && y < self.data[0].len();
        let (x, y) = start;
        for direction in 0..4 {
            let (nx, ny) = match direction {
                0 => (x - 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y),
                3 => (x, y - 1),
                _ => panic!("Invalid direction"),
            };
            if in_range(nx, ny) && self.data[nx][ny].pipe_open((direction + 2) % 4) {
                queue.push_back(((nx, ny), 1));
            }
        }
        visited[x][y] = true;
        //BFS
        let mut max_distance = 0;
        while let Some(((x, y), distance)) = queue.pop_front() {
            if x >= self.data.len() || y >= self.data[0].len() || visited[x][y] {
                continue;
            }
            max_distance = max_distance.max(distance);
            visited[x][y] = true;
            if let Piece::Pipe(pipe) = &self.data[x][y] {
                for direction in 0..4 {
                    let (nx, ny) = match direction {
                        0 => (x - 1, y),
                        1 => (x, y + 1),
                        2 => (x + 1, y),
                        3 => (x, y - 1),
                        _ => panic!("Invalid direction"),
                    };
                    if pipe.entries[direction]
                        && in_range(nx, ny)
                        && self.data[nx][ny].pipe_open((direction + 2) % 4)
                    {
                        queue.push_back(((nx, ny), distance + 1));
                    }
                }
            }
        }
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                if start == (x, y) {
                    print!("S");
                    continue;
                }
                print!("{}", if visited[x][y] { "X" } else { " " });
            }
            println!();
        }
        max_distance
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    for x in 0..data.data.len() {
        for y in 0..data.data[x].len() {
            if data.data[x][y] == Piece::Start {
                return data.furthest_distance((x, y)).to_string();
            }
        }
    }
    panic!("No start found");
}

pub fn part2(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    //     "..F7.
    // .FJ|.
    // SJ.L7
    // |F--J
    // LJ..."
    "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
}

fn import(lines: &Vec<String>) -> Day10Data {
    for l in lines.clone() {
        println!("{}", l);
    }
    println!("===");

    //probably not needed half the time
    Day10Data {
        data: lines
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().bytes().map(Piece::new).collect())
            .collect(),
    }
}
