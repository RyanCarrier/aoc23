use std::collections::HashMap;

use crate::util::Problem;

pub const DAY19: Problem = Problem {
    day: 19,
    part1,
    part2,
    test_data: Some(test_data),
};
enum Category {
    X = 0,
    M,
    A,
    S,
}
impl Category {
    fn new(input: char) -> Self {
        match input {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("Invalid category"),
        }
    }
}
enum Result<'a> {
    Category(&'a str),
    Reject,
    Accept,
}
impl Result<'_> {
    fn new(input: &str) -> Self {
        match input.len() {
            0 => panic!("Invalid result"),
            1 => match input {
                "R" => Result::Reject,
                "A" => Result::Accept,
                _ => Result::Category(input),
            },
            _ => Result::Category(input),
        }
    }
}
struct Rule<'a> {
    category: Category,
    lt: bool,
    value: usize,
    result: Result<'a>,
}
impl<'a> Rule<'a> {
    fn new(input: &str) -> Self {
        let mut chars = input.chars();
        let category = Category::new(chars.next().unwrap());
        let lt = match chars.next().unwrap() {
            '<' => true,
            '>' => false,
            _ => panic!("Invalid rule"),
        };
        let (num_str, result_str) = input[2..].split_once(":").unwrap();
        let value = num_str.parse().unwrap();
        let result = Result::new(result_str);
        Rule {
            category,
            lt,
            value,
            result,
        }
    }
}
struct RuleSet<'a> {
    rules: Vec<Rule<'a>>,
    result: Result<'a>,
}
impl<'a> RuleSet<'_> {
    fn new(input: &str) -> (&'a str, Self) {
        let (name, remaining) = input.split_once(":").unwrap();
        let mut raw_rules: Vec<&str> = remaining[0..remaining.len() - 1].split(",").collect();
        (
            name,
            RuleSet {
                result: Result::new(raw_rules.pop().unwrap()),
                rules: raw_rules.iter().map(Rule::new),
            },
        )
    }
}
type Input = [usize; 4];

struct Day19Data {
    rules: HashMap<&str, RuleSet>,
    inputs: Vec<Input>,
}
impl Day19Data {
    fn new(lines: &Vec<String>) -> Self {
        let mut trimmed: Vec<&String> = lines.iter().map(|x| x.trim()).collect();
        let split = trimmed.iter().position(|x| x.is_empty()).unwrap();
        let (raw_rules, raw_inputs) = trimmed.split_at(split);
        Day19Data {
            rules: raw_rules
                .iter()
                .map(|x| x.split(": "))
                .map(|mut x| {
                    let category = match x.next().unwrap() {
                        "x" => Category::X,
                        "m" => Category::M,
                        "a" => Category::A,
                        "s" => Category::S,
                        _ => panic!("Invalid category"),
                    };
                    let mut rules = x.next().unwrap().split(" | ");
                    let mut rule_set = RuleSet {
                        rules: Vec::new(),
                        result: Result::Reject,
                    };
                    for rule in rules {
                        let mut rule = rule.split(" ");
                        let lt = match rule.next().unwrap() {
                            "<" => true,
                            ">" => false,
                            _ => panic!("Invalid rule"),
                        };
                        let value = rule.next().unwrap().parse().unwrap();
                        let result = match rule.next().unwrap() {
                            "x" => Result::Category("x"),
                            "m" => Result::Category("m"),
                            "a" => Result::Category("a"),
                            "s" => Result::Category("s"),
                            "R" => Result::Reject,
                            "A" => Result::Accept,
                            _ => panic!("Invalid result"),
                        };
                        rule_set.rules.push(Rule {
                            category,
                            lt,
                            value,
                            result,
                        });
                    }
                    (category, rule_set)
                })
                .collect(),
            inputs: raw_inputs.iter().map(|x| {
                let mut split = x.split(",");
                [
                    split.next().unwrap()[2..].parse().unwrap(),
                    split.next().unwrap()[2..].parse().unwrap(),
                    split.next().unwrap()[2..].parse().unwrap(),
                    split.next().unwrap()[2..].parse().unwrap(),
                ]
            }),
        }
    }
}

pub fn part1(lines: &Vec<String>) -> String {
    let data = import(lines);
    "".to_owned()
}

pub fn part2(lines: &Vec<String>) -> String {
    let _ = import(lines);
    "".to_owned()
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

fn import(lines: &Vec<String>) -> Day17Data {
    let mut trimmed = lines.iter().map(|x| x.trim());
    let split = trimmed.position(|x| x.is_empty());
    Day19Data {
        rules: trimmed
            .take(split.unwrap())
            .map(|x| x.split(": "))
            .map(|mut x| {
                let category = match x.next().unwrap() {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => panic!("Invalid category"),
                };
                let mut rules = x.next().unwrap().split(" | ");
                let mut rule_set = RuleSet {
                    rules: Vec::new(),
                    result: Result::Reject,
                };
                for rule in rules {
                    let mut rule = rule.split(" ");
                    let lt = match rule.next().unwrap() {
                        "<" => true,
                        ">" => false,
                        _ => panic!("Invalid rule"),
                    };
                    let value = rule.next().unwrap().parse().unwrap();
                    let result = match rule.next().unwrap() {
                        "x" => Result::Category("x"),
                        "m" => Result::Category("m"),
                        "a" => Result::Category("a"),
                        "s" => Result::Category("s"),
                        "R" => Result::Reject,
                        "A" => Result::Accept,
                        _ => panic!("Invalid result"),
                    };
                    rule_set.rules.push(Rule {
                        category,
                        lt,
                        value,
                        result,
                    });
                }
                (category, rule_set)
            })
            .collect(),
        inputs: todo!(),
    }
}
