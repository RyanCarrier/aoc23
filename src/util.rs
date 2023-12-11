use std::{collections::BinaryHeap, fs, path::Path, sync::Arc, time::Duration};

use reqwest::{cookie::Jar, Url};
#[derive(Clone, Copy)]
pub struct Problem {
    pub day: usize,
    pub part1: fn(input: &Vec<String>) -> String,
    pub part2: fn(input: &Vec<String>) -> String,
    pub test_data: Option<fn() -> &'static str>,
}
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
    let session_id = fs::read_to_string("./cookie.txt").unwrap();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day)
        .parse::<Url>()
        .unwrap();
    let cookie = format!("session={}", session_id);
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);
    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    let resp = client
        .get(url)
        // .header("cookie", format!("session={}", cookie))
        .send()
        .unwrap();
    let body = resp.text().unwrap();
    body.split('\n').map(|x| x.to_owned()).collect()
}

#[allow(dead_code)]
pub struct Graph {
    pub verticies: usize,
    pub arcs: Vec<Vec<VectorArc>>,
}
pub struct VectorArc {
    pub from: usize,
    pub to: usize,
    pub weight: usize,
}
#[allow(dead_code)]
impl VectorArc {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }
}
#[derive(Copy, Clone, Eq, Debug)]
#[allow(dead_code)]
struct Vertex {
    index: usize,
    dist: usize,
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
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
#[allow(dead_code)]
impl Graph {
    pub fn dijkstra(&self, from: usize, to: usize) -> usize {
        let mut dist = vec![usize::MAX; self.verticies];
        let mut pq: BinaryHeap<Vertex> = BinaryHeap::new();
        dist[from] = 0;
        pq.push(Vertex {
            index: from,
            dist: 0,
        });
        while let Some(next) = pq.pop() {
            println!("{:?}", next);
            if next.index == to {
                return next.dist;
            }
            for arc in self.arcs[next.index].iter() {
                let new_dist = next.dist + arc.weight;
                println!(
                    "{}=={}->{} newdist {} dist[] {}",
                    next.index, arc.from, arc.to, new_dist, dist[arc.to]
                );
                if new_dist < dist[arc.to] {
                    dist[arc.to] = new_dist;
                    pq.push(Vertex {
                        index: arc.to,
                        dist: new_dist,
                    });
                }
            }
            println!("{:?}", pq);
        }
        0
    }
}
//test for dijkstra
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = Graph {
            verticies: 6,
            arcs: vec![
                vec![VectorArc::new(0, 1, 1), VectorArc::new(0, 2, 12)],
                vec![
                    VectorArc::new(1, 2, 9),
                    VectorArc::new(1, 3, 3),
                    VectorArc::new(1, 2, 1),
                ],
                vec![VectorArc::new(2, 4, 5)],
                vec![
                    VectorArc::new(3, 2, 4),
                    VectorArc::new(3, 4, 13),
                    VectorArc::new(3, 5, 15),
                ],
                vec![VectorArc::new(4, 5, 4)],
                vec![],
            ],
        };
        assert_eq!(graph.dijkstra(0, 5), 11);
        assert_eq!(graph.dijkstra(0, 2), 2);
    }
}
