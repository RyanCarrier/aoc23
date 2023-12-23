use std::collections::BinaryHeap;

use crate::util::Problem;

pub const DAY17: Problem = Problem {
    day: 17,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day17Data {
    data: Vec<Vec<usize>>,
}
impl Day17Data {
    #[allow(dead_code)]
    fn print_path(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        best_verticies: &Vec<(usize, usize)>,
    ) {
        let mut current = to;
        let mut path = vec![];
        path.push(current);
        let mut i = 0;
        while current != from {
            let (by, bx) = best_verticies[i];
            while current != (by, bx) {
                match current.0.cmp(&by) {
                    std::cmp::Ordering::Less => current.0 += 1,
                    std::cmp::Ordering::Greater => current.0 -= 1,
                    std::cmp::Ordering::Equal => {}
                }
                match current.1.cmp(&bx) {
                    std::cmp::Ordering::Less => current.1 += 1,
                    std::cmp::Ordering::Greater => current.1 -= 1,
                    std::cmp::Ordering::Equal => {}
                }
                path.push(current);
            }
            i += 1;
        }
        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                if path.contains(&(y, x)) {
                    print!("#");
                } else {
                    print!("*");
                }
            }
            println!();
        }
    }

    pub fn dijkstra_day17(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        min_steps: usize,
        max_steps: usize,
    ) -> (usize, Vec<(usize, usize)>) {
        // println!("from: {:?}, to: {:?}", from, to);
        let to_yx = |d: usize| (d / self.data[0].len(), d % self.data[0].len());
        let from_yx = |y: usize, x: usize| y * self.data[0].len() + x;
        let bounds_ok = |yx: (usize, usize)| yx.0 < self.data.len() && yx.1 < self.data[0].len();
        let to = from_yx(to.0, to.1);

        let (x, y) = from;
        // println!("from: {:?}, {:?}", from, to_yx(from_yx(y, x)),);
        // println!("to: {}, {:?}", to, to_yx(to));
        let max_vertex = Vertex {
            index: usize::MAX,
            dist: usize::MAX,
            was_horizontal: false,
            best_index: usize::MAX,
        };

        let mut was_vertical_dist = vec![vec![max_vertex; self.data[0].len()]; self.data.len()];
        let mut was_horizontal_dist = vec![vec![max_vertex; self.data[0].len()]; self.data.len()];
        let mut pq: BinaryHeap<Vertex> = BinaryHeap::new();
        let vert_from = Vertex {
            index: from_yx(y, x),
            dist: 0,
            was_horizontal: false,
            best_index: 0,
        };
        let hor_from = Vertex {
            index: from_yx(y, x),
            dist: 0,
            was_horizontal: true,
            best_index: 0,
        };
        was_vertical_dist[y][x] = vert_from;
        was_horizontal_dist[y][x] = hor_from;
        was_vertical_dist[y][x].dist = usize::MAX;
        was_horizontal_dist[y][x].dist = usize::MAX;
        pq.push(vert_from);
        pq.push(hor_from);

        while let Some(current) = pq.pop() {
            // println!("current: {:?}", current);
            // println!("pq: {:?}", pq.iter().map(|x| x.dist).collect::<Vec<_>>());
            let (next_dist_ref, current_dist_ref) = if current.was_horizontal {
                //the next one will be a vertical
                (&mut was_vertical_dist, &mut was_horizontal_dist)
            } else {
                (&mut was_horizontal_dist, &mut was_vertical_dist)
            };
            // println!("{:?}", current);
            let (y, x) = to_yx(current.index);
            if current.dist > current_dist_ref[y][x].dist {
                // println!(
                //     "skipping {:?}, {} {}, {}",
                //     (y, x),
                //     current.dist,
                //     current_dist_ref[y][x].dist,
                //     current.was_horizontal
                // );
                continue;
            }
            if current.index == to {
                let mut best_path = vec![];
                let final_dist = current.dist;
                let mut step = current;
                let from_index = from_yx(from.0, from.1);
                while step.index != from_index {
                    let yx = to_yx(step.index);
                    // println!("step: {:?}, {}", yx, step.index);
                    best_path.push(yx);
                    let best_yx = to_yx(step.best_index);
                    step = if step.was_horizontal {
                        was_vertical_dist[best_yx.0][best_yx.1]
                    } else {
                        was_horizontal_dist[best_yx.0][best_yx.1]
                    };
                }
                best_path.push(to_yx(step.index));
                return (final_dist, best_path);
            }
            let mut arcs = vec![];
            let mut sum = (0, 0);
            let mut lower = (y, x);
            let mut upper = (y, x);
            for i in 1..=max_steps {
                if current.was_horizontal {
                    lower.0 -= 1;
                    upper.0 += 1;
                } else {
                    lower.1 -= 1;
                    upper.1 += 1;
                };
                if bounds_ok(lower) {
                    sum.0 += self.data[lower.0][lower.1];
                    if i >= min_steps {
                        arcs.push((lower, sum.0));
                    }
                }
                if bounds_ok(upper) {
                    sum.1 += self.data[upper.0][upper.1];
                    if i >= min_steps {
                        arcs.push((upper, sum.1));
                    }
                }
            }
            for ((to_y, to_x), weight) in arcs.iter() {
                let new_dist = current.dist + weight;
                if new_dist < next_dist_ref[*to_y][*to_x].dist {
                    let arc_vertex = Vertex {
                        index: from_yx(*to_y, *to_x),
                        dist: new_dist,
                        was_horizontal: !current.was_horizontal,
                        best_index: current.index,
                    };
                    next_dist_ref[*to_y][*to_x] = arc_vertex;
                    pq.push(arc_vertex);
                }
            }
        }
        println!("No path to exit found");
        println!(
            "{:?}, {:?}, {:?}",
            to,
            to_yx(to),
            from_yx(to_yx(to).0, to_yx(to).1)
        );
        // (usize::MAX, best_vertex)
        (usize::MAX, vec![])
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    let to = (data.data.len() - 1, data.data[0].len() - 1);
    let from = (0, 0);
    let (distance, _) = data.dijkstra_day17(from, to, 0, 3);
    // let (distance, best_verticies) = data.dijkstra_day17(from, to);
    // data.print_path(from, to, &best_verticies);
    distance.to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut data = import(lines);
    let to = (data.data.len() - 1, data.data[0].len() - 1);
    let from = (0, 0);
    let (distance, _) = data.dijkstra_day17(from, to, 4, 10);
    // data.print_path(from, to, &best_verticies);
    distance.to_string()
}
pub fn test_data() -> &'static str {
    "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
}
#[derive(Copy, Clone, Eq, Debug)]
#[allow(dead_code)]
pub struct Vertex {
    pub index: usize,
    pub dist: usize,
    pub was_horizontal: bool,
    pub best_index: usize,
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.was_horizontal == other.was_horizontal
    }
}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
        // other.dist.cmp(&self.dist)
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn import(lines: &Vec<String>) -> Day17Data {
    Day17Data {
        data: lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().bytes().map(|x| (x - b'0') as usize).collect())
            .collect(),
    }
}
