use std::ops::RangeInclusive;

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
impl Day5Data {
    fn collapse_mappings(&self) -> Vec<Mapping> {
        let mut final_result: Vec<Mapping> = Vec::new();
        for i in 0..(self.maps.len()) {
            let result: Vec<Mapping> = final_result.clone();
            final_result.clear();
            let layer = &self.maps[i];
            let mut from_cursor = 0;
            let max_layer_from = layer[layer.len() - 1].from_max();
            let max_result_from = if result.len() > 0 {
                result[result.len() - 1].from_max()
            } else {
                0
            };
            // let mut to_cursor = 0;
            let mut result_index = 0;
            let get_layer_map = |from: usize| -> Option<&Mapping> {
                let filtered = layer
                    .iter()
                    .filter(|mapping| from >= mapping.from && from < mapping.from_max())
                    .collect::<Vec<&Mapping>>();
                match filtered.len() {
                    0 => None,
                    1 => Some(filtered[0]),
                    _ => panic!("Maps should not overlap"),
                }
            };
            let get_layer_next_map = |from: usize| -> Option<&Mapping> {
                if layer.len() == 0 {
                    return None;
                }
                if from < layer[0].from {
                    return Some(&layer[0]);
                }
                for i in 0..layer.len() - 1 {
                    if from >= layer[i].from_max() && from < layer[i + 1].from {
                        return Some(&layer[i + 1]);
                    }
                }
                return None;
            };
            while from_cursor < max_layer_from || from_cursor < max_result_from {
                //--Options--
                //direct->direct
                //direct->map
                //map->direct
                //map->map
                while result_index < result.len() && from_cursor >= result[result_index].from_max()
                {
                    result_index += 1;
                }
                if result_index == result.len() || from_cursor < result[result_index].from {
                    //we don't need to check result_index-1 as we won't grow result index unless
                    //necessary
                    //direct->?
                    if let Some(m) = get_layer_map(from_cursor) {
                        // println!("Got layer map");
                        //direct->map
                        let sub_range = if result_index < result.len() {
                            m.range.min(result[result_index].from - from_cursor)
                            //range us up to the next result map or next layer map
                        } else {
                            //the range minus how far we are into the next layer map
                            //likely from_cursor==m.from
                            m.range - (from_cursor - m.from)
                        };
                        final_result.push(Mapping {
                            from: from_cursor,
                            to: m.map(&from_cursor),
                            range: sub_range,
                        });
                        from_cursor += sub_range;
                    } else if let Some(m) = get_layer_next_map(from_cursor) {
                        // println!("Got next layer map");
                        //direct->direct
                        from_cursor = if result_index < result.len() {
                            m.from.min(result[result_index].from)
                        } else {
                            m.from
                        };
                    } else {
                        panic!("Got NO layer map");
                    }
                } else if from_cursor >= result[result_index].from
                    && from_cursor < result[result_index].from_max()
                {
                    //map->?
                    while from_cursor < result[result_index].from_max() {
                        //handle until we are out of the current resultmap
                        let mut mapped_from_cursor = result[result_index].map(&from_cursor);
                        let result_range =
                            result[result_index].range - (from_cursor - result[result_index].from);
                        let sub_range: usize;
                        if let Some(m) = get_layer_map(mapped_from_cursor) {
                            //map->map
                            sub_range = result_range.min(m.range - (mapped_from_cursor - m.from));
                            mapped_from_cursor = m.map(&mapped_from_cursor);
                        } else {
                            //map->direct
                            sub_range = if let Some(m) = get_layer_next_map(mapped_from_cursor) {
                                result_range.min(m.from - mapped_from_cursor)
                            } else {
                                result_range
                            };
                        }
                        final_result.push(Mapping {
                            from: from_cursor,
                            to: mapped_from_cursor,
                            range: sub_range,
                        });
                        from_cursor += sub_range;
                    }
                }
            }
        }
        final_result
    }
}
#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Mapping {
    from: usize,
    to: usize,
    range: usize,
}
impl Mapping {
    fn contains(&self, i: &usize) -> bool {
        self.from <= *i && *i < self.from + self.range
    }
    fn map(&self, i: &usize) -> usize {
        self.to + (i - self.from)
    }
    fn from_max(&self) -> usize {
        self.from + self.range
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let locations = data.seeds.iter().map(|s| {
        let mut i = *s;
        for map in &data.maps {
            for m in map {
                if m.contains(&i) {
                    i = m.map(&i);
                    break;
                }
            }
            //i=i, (no mapping found)
        }
        i
    });
    format!("{:?}", locations.min().unwrap(),)
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let mapping = data.collapse_mappings();
    let new_seeds: Vec<RangeInclusive<usize>> = (0..data.seeds.len())
        .step_by(2)
        .map(|i| data.seeds[i]..=(data.seeds[i] + data.seeds[i + 1]))
        .collect();
    let mut min = usize::MAX;
    let mut i;
    for seed_set in new_seeds {
        i = *seed_set.start();
        for j in 0..mapping.len() {
            let map = mapping[j];
            if !map.contains(&i) {
                continue;
            }
            min = min.min(map.map(&i));
            if j < mapping.len() - 1 {
                i = mapping[j + 1].from;
                if i >= *seed_set.end() {
                    break;
                }
            }
        }
    }
    format!("{}", min)
}
pub fn test_data() -> &'static str {
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
            from,
            to,
            range: range_len,
        });
    }
    maps.iter_mut().for_each(|x| x.sort());
    Day5Data { seeds, maps }
}
