use crate::util::Problem;

pub const DAY1: Problem = Problem {
    day: 1,
    part1,
    part2,
    test_data,
};

pub fn part1(lines: Vec<String>) -> String {
    let mut result = [false; 2020];
    for l in lines {
        let n: i32 = l.parse().unwrap();
        if result[2020 - n as usize] {
            return format!("{}", (2020 - n) * n);
        } else {
            result[n as usize] = true;
        }
    }
    return "".to_owned();
}

pub fn part2(lines: Vec<String>) -> String {
    let target = 2020;
    let mut nums: Vec<i32> = lines
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    nums.sort();
    for i in 0..nums.len() {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == target {
                return format!("{}", nums[i] * nums[left] * nums[right]);
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    return "".to_owned();
}
pub fn test_data() -> Option<String> {
    Some(
        "1721
979
366
299
675
1456"
            .to_owned(),
    )
}

// fn import(lines: Vec<String>) -> Day0Data {
//probably not needed half the time
// Day0Data {
//     data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
// }
// }
