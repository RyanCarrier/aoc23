use crate::util::Problem;
use std::fmt::Debug;

pub const DAY13: Problem = Problem {
    day: 13,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day13Data {
    patterns: Vec<Pattern>,
}
struct Pattern {
    pattern: Vec<Vec<bool>>,
}
impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //lmao
        write!(f, "  ")?;
        for j in 0..self.pattern[0].len() {
            write!(f, "{}", j % 10)?;
        }
        for (i, row) in self.pattern.iter().enumerate() {
            write!(f, "\n{} ", i % 10)?;
            for col in row {
                write!(f, "{}", if *col { '#' } else { '.' })?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}
impl Pattern {
    fn fold_row(&self, allow_smudge: bool) -> usize {
        // Self::fold_generic(&self.pattern)
        Self::fold_smudge(&self.pattern, allow_smudge)
    }
    fn fold_col(&self, allow_smudge: bool) -> usize {
        let mut flipped = vec![vec![false; self.pattern.len()]; self.pattern[0].len()];
        for i in 0..self.pattern.len() {
            for j in 0..self.pattern[i].len() {
                flipped[j][i] = self.pattern[i][j];
            }
        }
        // Self::fold_generic(&flipped)
        Self::fold_smudge(&flipped, allow_smudge)
    }

    fn fold(&self, allow_smudge: bool) -> usize {
        let rows = self.fold_row(allow_smudge);
        if rows > 0 {
            return rows * 100;
        }
        self.fold_col(allow_smudge)
    }

    fn fold_smudge(pattern: &Vec<Vec<bool>>, allow_smudge: bool) -> usize {
        let one_error = |a: &Vec<bool>, b: &Vec<bool>| -> bool {
            a.iter()
                .zip(b.iter())
                .fold(0, |acc, (a, b)| if *a == *b { acc } else { acc + 1 })
                == 1
        };
        let row_count: Vec<usize> = pattern
            .iter()
            .map(|r| r.iter().filter(|x| **x).count())
            .collect();
        let l = pattern.len();
        'pivot: for pivot in 0..l - 1 {
            let mut i = pivot;
            let mut j = pivot + 1;
            let mut mistake = false;
            loop {
                if row_count[i] != row_count[j] || *pattern[i] != *pattern[j] {
                    if allow_smudge
                        && !mistake
                        && row_count[i].abs_diff(row_count[j]) == 1
                        && one_error(&pattern[i], &pattern[j])
                    {
                        mistake = true;
                    } else {
                        continue 'pivot;
                    }
                }
                if i == 0 || j == l - 1 {
                    if !allow_smudge || mistake {
                        return pivot + 1;
                    }
                    break;
                }
                i -= 1;
                j += 1;
            }
        }
        0
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let result: usize = data.patterns.iter().map(|p| p.fold(false)).sum::<usize>();
    format!("{:?}", result)
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let result: usize = data.patterns.iter().map(|p| p.fold(true)).sum::<usize>();
    format!("{:?}", result)
}
pub fn test_data() -> &'static str {
    "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
}

fn import(lines: &Vec<String>) -> Day13Data {
    let patterns: Vec<Vec<String>> = lines.split(|l| l.is_empty()).map(|l| l.to_vec()).collect();
    Day13Data {
        patterns: patterns
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| Pattern {
                pattern: p
                    .iter()
                    .map(|l| l.trim().chars().map(|c| c == '#').collect())
                    .collect(),
            })
            .collect(),
    }
}
