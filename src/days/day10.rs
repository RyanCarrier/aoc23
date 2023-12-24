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
//use SWAP_PIPES to know if we are swapping whether we are inside or not when passing through
const SWAP_PIPES: [[bool; 4]; 3] = [PIPES_INSTANCE[0], PIPES_INSTANCE[2], PIPES_INSTANCE[3]];

#[derive(PartialEq, Clone, Copy)]
struct Pipe {
    entries: [bool; 4],
}

#[derive(PartialEq, Copy, Clone)]
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
    fn pipe_open(&self, direction: usize) -> bool {
        match self {
            Piece::Pipe(pipe) => pipe.entries[direction],
            _ => false,
        }
    }
}
impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Pipe(pipe) => {
                let mut index = PIPES_INSTANCE.iter().enumerate().filter_map(|(i, x)| {
                    if x == &pipe.entries {
                        Some(i)
                    } else {
                        None
                    }
                });
                write!(f, "{}", PIPES[index.next().unwrap() as usize] as char)
            }
            Piece::Empty => write!(f, "."),
            Piece::Start => write!(f, "S"),
        }
    }
}

impl Pipe {
    fn new(input: u8) -> Pipe {
        Pipe {
            entries: PIPES_INSTANCE[PIPES.iter().position(|&x| x == input).unwrap()],
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
    fn start_point(&self) -> (usize, usize) {
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                if self.data[x][y] == Piece::Start {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }
    fn furthest_distance(&self, start: (usize, usize)) -> (usize, Vec<Vec<bool>>) {
        let mut visited = vec![vec![false; self.data[0].len()]; self.data.len()];
        let mut queue = VecDeque::new();
        let in_range = |x: usize, y: usize| x < self.data.len() && y < self.data[0].len();
        let match_direction = |direction: usize, x: usize, y: usize| match direction {
            0 => (x - 1, y),
            1 => (x, y + 1),
            2 => (x + 1, y),
            3 => (x, y - 1),
            _ => panic!("Invalid direction"),
        };
        let (x, y) = start;
        //in after part 2 this should just replace S with the actual pipe but whatever
        for direction in 0..4 {
            let (nx, ny) = match_direction(direction, x, y);
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
                    let (nx, ny) = match_direction(direction, x, y);
                    if pipe.entries[direction]
                        && in_range(nx, ny)
                        && self.data[nx][ny].pipe_open((direction + 2) % 4)
                    {
                        queue.push_back(((nx, ny), distance + 1));
                    }
                }
            }
        }
        // for x in 0..self.data.len() {
        //     for y in 0..self.data[x].len() {
        //         if start == (x, y) {
        //             print!("S");
        //             continue;
        //         }
        //         print!("{}", if visited[x][y] { "X" } else { " " });
        //     }
        //     println!();
        // }
        (max_distance, visited)
    }
    fn get_loop_only(&self) -> Vec<Vec<Piece>> {
        let start = self.start_point();
        let (_, visited) = self.furthest_distance(start);
        let mut data: Vec<Vec<Piece>> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, y)| {
                        if visited[i][j] {
                            y.clone()
                        } else {
                            Piece::Empty
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Piece>>>();
        let (x, y) = start;
        let mut open_pipes = [false; 4];
        for direction in 0..4 {
            let (nx, ny) = match direction {
                0 => (x - 1, y),
                1 => (x, y + 1),
                2 => (x + 1, y),
                3 => (x, y - 1),
                _ => panic!("Invalid direction"),
            };
            if nx >= self.data.len() || ny >= self.data[0].len() {
                continue;
            }
            open_pipes[direction] = match self.data[nx][ny] {
                Piece::Pipe(pipe) => pipe.entries[(direction + 2) % 4],
                _ => false,
            };
        }
        data[x][y] = Piece::Pipe(Pipe {
            entries: open_pipes,
        });
        data
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    return data.furthest_distance(data.start_point()).0.to_string();
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let loop_only = data.get_loop_only();
    let result = loop_only.iter().fold(0, |total_inside, l| {
        let mut inside = false;
        // println!("");
        total_inside
            + l.iter().fold(0, |inside_amount, y| {
                if let Piece::Pipe(pipe) = y {
                    if SWAP_PIPES.contains(&pipe.entries) {
                        inside = !inside;
                    }
                    // print!(" ");
                } else {
                    // print!("{}", if inside { "I" } else { " " });
                    if inside {
                        return inside_amount + 1;
                    }
                }
                inside_amount
            })
    });
    format!("{}", result)
}
pub fn test_data() -> &'static str {
    //     "..F7.
    // .FJ|.
    // SJ.L7
    // |F--J
    // LJ..."
    //     "-L|F7
    // 7S-7|
    // L|7||
    // -L-J|
    // L|-JF"
    "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
}

fn import(lines: &Vec<String>) -> Day10Data {
    // for l in lines.clone() {
    //     println!("{}", l);
    // }
    // println!("===");
    Day10Data {
        data: lines
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().bytes().map(Piece::new).collect())
            .collect(),
    }
}
