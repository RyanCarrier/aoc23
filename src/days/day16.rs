use crate::util::Problem;

pub const DAY16: Problem = Problem {
    day: 16,
    part1,
    part2,
    test_data: Some(test_data),
};
#[derive(Clone)]
struct Day16Data {
    data: Vec<Vec<Mirror>>,
    //N, E, S, W
    memo: Vec<Vec<[bool; 4]>>,
}
impl Day16Data {
    fn start(&mut self) {
        self.step((0, 0), 1)
    }
    fn result(&self) -> usize {
        self.memo
            .iter()
            .flatten()
            .filter(|x| x.iter().any(|y| *y))
            .count()
    }
    fn step(&mut self, start: (usize, usize), dir: usize) {
        let (mut y, mut x) = start;
        loop {
            if x >= self.data[0].len() || y >= self.data.len() || self.memo[y][x][dir] {
                return;
            }
            self.memo[y][x][dir] = true;
            match self.data[y][x] {
                Mirror::Vertical => {
                    if dir % 2 != 0 {
                        return self.split(y, x, 0, 2);
                    }
                }
                Mirror::Horizontal => {
                    if dir % 2 != 1 {
                        return self.split(y, x, 3, 1);
                    }
                }
                Mirror::FwdDiagonal => {
                    return self.dir_step(y, x, dir ^ 1);
                }
                Mirror::BwdDiagonal => {
                    return self.dir_step(y, x, 3 - dir);
                }
                Mirror::None => (),
            }
            (y, x) = self.xy_dir(y, x, dir);
        }
    }
    fn split(&mut self, y: usize, x: usize, dir: usize, dir2: usize) {
        self.dir_step(y, x, dir);
        self.dir_step(y, x, dir2);
    }
    fn dir_step(&mut self, y: usize, x: usize, dir: usize) {
        self.step(self.xy_dir(y, x, dir), dir)
    }
    fn xy_dir(&self, y: usize, x: usize, dir: usize) -> (usize, usize) {
        match dir {
            0 => (y - 1, x),
            1 => (y, x + 1),
            2 => (y + 1, x),
            3 => (y, x - 1),
            _ => panic!("Invalid direction"),
        }
    }
    #[allow(dead_code)]
    fn print(&self) {
        self.memo.iter().for_each(|l| {
            println!("");
            l.iter().for_each(|x| {
                print!(
                    "{}",
                    if x[0] || x[1] || x[2] || x[3] {
                        'X'
                    } else {
                        '.'
                    }
                );
            });
        });
        println!("\n==========");
    }
}

#[derive(Clone)]
enum Mirror {
    Vertical,
    Horizontal,
    FwdDiagonal,
    BwdDiagonal,
    None,
}
impl Mirror {
    fn new(input: &char) -> Mirror {
        match input {
            '|' => Mirror::Vertical,
            '-' => Mirror::Horizontal,
            '/' => Mirror::FwdDiagonal,
            '\\' => Mirror::BwdDiagonal,
            '.' => Mirror::None,
            _ => panic!("Invalid mirror type"),
        }
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    data.start();
    data.result().to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let original = import(lines);
    let mut best = 0;
    let ymax = original.data.len() - 1;
    let xmax = original.data[0].len() - 1;
    let mut try_best = |y: usize, x: usize, dir: usize| {
        let mut data = original.clone();
        data.step((y, x), dir);
        best = best.max(data.result())
    };
    for y in 0..original.data.len() {
        for (x, dir) in [(0, 1), (xmax, 3)] {
            try_best(y, x, dir);
        }
    }
    for x in 0..original.data[0].len() {
        for (y, dir) in [(0, 2), (ymax, 0)] {
            try_best(y, x, dir);
        }
    }
    best.to_string()
}
pub fn test_data() -> &'static str {
    ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
"
}

fn import(lines: &Vec<String>) -> Day16Data {
    let lines: Vec<&String> = lines.iter().filter(|x| !x.is_empty()).collect();
    let memo = vec![vec![[false; 4]; lines[0].len()]; lines.len()];
    let mut data = vec![vec![Mirror::None; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            data[i][j] = Mirror::new(&c);
        }
    }
    Day16Data { data, memo }
}
