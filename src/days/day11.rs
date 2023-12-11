use crate::util::Problem;

pub const DAY11: Problem = Problem {
    day: 11,
    part1,
    part2,
    test_data: Some(test_data),
};

struct Day11Data {
    galaxies: Vec<(usize, usize)>,
}
impl Day11Data {
    fn sum_distance_all(&self) -> usize {
        let mut total = 0;
        for i in 0..self.galaxies.len() {
            for j in (i + 1)..self.galaxies.len() {
                total += distance(&self.galaxies[i], &self.galaxies[j]);
            }
        }
        total
    }

    fn expand(&mut self, amount: usize) {
        let amount = if amount == 1 { 1 } else { amount - 1 };
        let mut xs: Vec<usize> = self.galaxies.iter().map(|(x, _)| *x).collect();
        let mut ys: Vec<usize> = self.galaxies.iter().map(|(_, y)| *y).collect();
        xs.sort();
        ys.sort();
        let x_expand: Vec<usize> = xs.windows(2).fold(Vec::new(), |mut acc, x| {
            for new in (x[0] + 1)..x[1] {
                acc.push(new);
            }
            acc
        });
        let y_expand: Vec<usize> = ys.windows(2).fold(Vec::new(), |mut acc, y| {
            for new in (y[0] + 1)..y[1] {
                acc.push(new);
            }
            acc
        });
        self.galaxies.iter_mut().for_each(|(x, y)| {
            *x += x_expand
                .iter()
                .filter(|x_expanding| *x > **x_expanding)
                .count()
                * amount;
            *y += y_expand
                .iter()
                .filter(|y_expanding| *y > **y_expanding)
                .count()
                * amount;
        });
    }
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let x = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let y = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    x + y
}

pub fn part1(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    data.expand(1);
    let total = data.sum_distance_all();
    format!("{}", total)
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    data.expand(1_000_000);
    // data.expand(100);
    let total = data.sum_distance_all();
    format!("{}", total)
}
pub fn test_data() -> &'static str {
    //  x0 x1 x2
    //y0
    //y1
    //y2
    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
}

fn import(lines: &Vec<String>) -> Day11Data {
    let mut galaxies = Vec::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        line.trim()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| {
                galaxies.push((x, y));
            })
    });
    Day11Data { galaxies }
}
