use std::ops::Range;

use crate::util::Problem;

pub const DAY5: Problem = Problem {
    day: 5,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day5Data {
    seeds: Vec<usize>,
    maps: Vec<Vec<Mapping>>,
}
#[derive(Clone)]
struct Mapping {
    from: Range<usize>,
    to: Range<usize>,
    range: usize,
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let locations = data.seeds.iter().map(|s| {
        let mut i = *s;
        'stage: for map in &data.maps {
            for m in map {
                if m.from.contains(&i) {
                    i = m.to.start + (i - m.from.start);
                    continue 'stage;
                }
            }
            //i=i, (no mapping found)
        }
        i
    });
    format!("{:?}", locations.min().unwrap())
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let mut new_seeds = Vec::new();
    for i in (0..data.seeds.len()).step_by(2) {
        println!("{} {}", data.seeds[i], data.seeds[i + 1]);
        let seed = data.seeds[i];
        let mut seed_iterator: Vec<usize> =
            (seed..=(seed + data.seeds[i + 1])).collect::<Vec<usize>>();
        // println!("==={:?}", new_seeds);
        new_seeds.append(&mut seed_iterator);
        // println!("+++{:?}", new_seeds);
    }
    // println!("{:?}", new_seeds);
    let locations = new_seeds.iter().map(|s| {
        let mut i = *s;
        'stage: for map in &data.maps {
            for m in map {
                if m.from.contains(&i) {
                    i = m.to.start + (i - m.from.start);
                    continue 'stage;
                }
            }
            //i=i, (no mapping found)
        }
        i
    });
    format!("{:?}", locations.min().unwrap())
}
pub fn test_data() -> String {
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
    .to_owned()
}

fn import(lines: &Vec<String>) -> Day5Data {
    //probably not needed half the time
    let mut lines = lines.iter();
    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut maps: Vec<Vec<Mapping>> = vec![Vec::new(); 7];
    let mut i = 0;
    lines.next();
    lines.next();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            i += 1;
            continue;
        }
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let to: usize = numbers[0].parse().expect("Failed to parse from");
        let from: usize = numbers[1].parse().expect("Failed to parse from");
        let range_len: usize = numbers[2].parse().expect("Failed to parse from");
        maps[i].push(Mapping {
            from: from..(from + range_len),
            to: to..(to + range_len),
            range: range_len,
        });
    }
    Day5Data { seeds, maps }
}
