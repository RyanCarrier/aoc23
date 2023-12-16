use crate::util::Problem;

pub const DAY16: Problem = Problem {
    day: 16,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day16Data {
    data: Vec<Vec<Mirror>>,
    //N, E, S, W
    memo: Vec<Vec<[bool; 4]>>,
}
impl Day16Data {
    fn start(&mut self) {
        self.step((0, 0), 1)
    }
    fn step(&mut self, start: (usize, usize), dir: usize) {
        let yx_ok =
            |y: usize, x: usize| x <= (self.data[0].len() - 1) && y <= (self.data.len() - 1);
        let (mut y, mut x) = start;
        if !yx_ok(y, x) {
            return;
        }
        if self.memo[y][x][dir] {
            return;
        }
        loop {
            if !yx_ok(y, x) {
                return;
            }
            self.memo[y][x][dir] = true;
            // self.print();
            match self.data[y][x] {
                Mirror::Vertical => {
                    if dir % 2 != 0 {
                        self.dir_step(y, x, 0);
                        self.dir_step(y, x, 2);
                        break;
                    }
                }
                Mirror::Horizontal => {
                    if dir % 2 != 1 {
                        self.dir_step(y, x, 3);
                        self.dir_step(y, x, 1);
                        break;
                    }
                }
                Mirror::FwdDiagonal => {
                    self.dir_step(y, x, dir ^ 1);
                    break;
                }
                Mirror::BwdDiagonal => {
                    self.dir_step(y, x, 3 - dir);
                    break;
                }
                Mirror::None => (),
            }
            (y, x) = self.xy_dir(y, x, dir);
        }
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
    data.memo
        .iter()
        .flatten()
        .filter(|x| x[0] || x[1] || x[2] || x[3])
        .count()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    // let data = import(lines);
    "".to_owned()
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
