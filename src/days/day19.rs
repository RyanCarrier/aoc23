use std::{collections::HashMap, ops::RangeInclusive};

use crate::util::Problem;

pub const DAY19: Problem = Problem {
    day: 19,
    part1,
    part2,
    test_data: Some(test_data),
};
#[derive(Clone, Eq, PartialEq)]
enum Result {
    Category(String),
    Reject,
    Accept,
}
impl Result {
    fn new(input: &str) -> Self {
        match input.len() {
            0 => panic!("Invalid result"),
            1 => match input {
                "R" => Result::Reject,
                "A" => Result::Accept,
                _ => Result::Category(input.to_string()),
            },
            _ => Result::Category(input.to_string()),
        }
    }
}
#[derive(Clone)]
struct Rule {
    category: usize,
    lt: bool,
    value: usize,
    result: Result,
}
impl Rule {
    fn new(input: &str) -> Self {
        let mut chars = input.chars();
        let category = "xmas".find(chars.next().unwrap()).unwrap();
        let lt = match chars.next().unwrap() {
            '<' => true,
            '>' => false,
            _ => panic!("Invalid rule"),
        };
        let (num_str, result_str) = input[2..].split_once(":").unwrap();
        Rule {
            category,
            lt,
            value: num_str.parse().unwrap(),
            result: Result::new(result_str),
        }
    }
    fn run(&self, input: Input) -> Option<Result> {
        // T T=T, F F=T, else F
        let is_lt = input[self.category as usize] < self.value;
        if !(self.lt ^ is_lt) {
            Some(self.result.clone())
        } else {
            None
        }
    }
}
struct RuleSet {
    rules: Vec<Rule>,
    result: Result,
}
impl RuleSet {
    fn new(input: &str) -> (String, Self) {
        let (name, remaining) = input.split_once("{").unwrap();
        let mut raw_rules: Vec<&str> = remaining[0..remaining.len() - 1].split(",").collect();
        (
            name.to_string(),
            RuleSet {
                result: Result::new(raw_rules.pop().unwrap()),
                rules: raw_rules.iter().map(|x| Rule::new(*x)).collect(),
            },
        )
    }
    fn run(&self, input: Input) -> Result {
        for rule in &self.rules {
            if let Some(result) = rule.run(input) {
                return result;
            }
        }
        self.result.clone()
    }
}
type Input = [usize; 4];

struct Day19Data {
    rules: HashMap<String, RuleSet>,
    inputs: Vec<Input>,
}
impl Day19Data {
    fn new(lines: &Vec<String>) -> Self {
        let trimmed: Vec<&str> = lines.iter().map(|x| x.trim()).collect();
        let split = trimmed.iter().position(|x| x.is_empty()).unwrap();
        let (raw_rules, raw_inputs) = trimmed.split_at(split);
        Day19Data {
            rules: raw_rules
                .iter()
                .map(|x| {
                    let (name, rule_set) = RuleSet::new(x);
                    (name, rule_set)
                })
                .collect(),
            inputs: raw_inputs[1..]
                .iter()
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let mut split = x[1..x.len() - 1].split(",");
                    [
                        split.next().unwrap()[2..].parse().unwrap(),
                        split.next().unwrap()[2..].parse().unwrap(),
                        split.next().unwrap()[2..].parse().unwrap(),
                        split.next().unwrap()[2..].parse().unwrap(),
                    ]
                })
                .collect(),
        }
    }
    fn range_split(
        &self,
        cat: &String,
        rule_number: usize,
        range: [RangeInclusive<usize>; 4],
    ) -> usize {
        let ruleset = &self.rules[cat];
        if rule_number >= ruleset.rules.len() {
            return match &ruleset.result {
                Result::Category(cat) => self.range_split(&cat, 0, range),
                Result::Reject => 0,
                Result::Accept => range.iter().map(|r| r.end() - r.start() + 1).product(),
            };
        }
        let rule = &ruleset.rules[rule_number];
        let range_value = &range[rule.category];
        if (rule.lt && rule.value <= *range_value.start())
            || (!rule.lt && rule.value >= *range_value.end())
        {
            //failed whole range
            return self.range_split(cat, rule_number + 1, range);
        }
        if (rule.lt && rule.value > *range_value.end())
            || (!rule.lt && rule.value < *range_value.start())
        {
            //passed whole range
            return match &rule.result {
                Result::Category(cat) => self.range_split(cat, 0, range),
                Result::Reject => 0,
                Result::Accept => range.iter().map(|r| r.end() - r.start() + 1).product(),
            };
        }
        let mut pass_range = range.clone();
        let mut fail_range = range.clone();
        if rule.lt {
            pass_range[rule.category] = *range[rule.category].start()..=rule.value - 1;
            fail_range[rule.category] = rule.value..=*range[rule.category].end();
        } else {
            fail_range[rule.category] = *range[rule.category].start()..=rule.value;
            pass_range[rule.category] = rule.value + 1..=*range[rule.category].end();
        }
        return self.range_split(cat, rule_number, pass_range)
            + self.range_split(cat, rule_number, fail_range);
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    let mut total = 0;
    for input in data.inputs {
        let mut result = data.rules["in"].run(input);
        loop {
            match result {
                Result::Category(name) => {
                    result = data.rules[&name].run(input);
                }
                Result::Reject => break,
                Result::Accept => break total += input.iter().sum::<usize>(),
            }
        }
    }
    total.to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let data = import(lines);
    let default = 1_usize..=4000_usize;
    let input_ranges: [RangeInclusive<usize>; 4] = [
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
    ];
    data.range_split(&"in".to_string(), 0, input_ranges)
        .to_string()
}
pub fn test_data() -> &'static str {
    "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
}

fn import(lines: &Vec<String>) -> Day19Data {
    Day19Data::new(lines)
}
