use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";

#[derive(Debug)]
enum Status {
    Accepted,
    Rejected,
}

impl Status {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Accepted,
            'R' => Self::Rejected,
            _ => unreachable!("Invalid status"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Category {
    ExtremelyCoolLooking, // x
    Musical,              // m
    Aerodynamic,          // a
    Shiny,                // s
}

impl Category {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::ExtremelyCoolLooking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => unreachable!("Invalid category"),
        }
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

struct Workflow {}

fn main() {
    println!("Part 1: {}", part_1(TEST));
}

fn part_1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let mut workflow_map: HashMap<_, _> = HashMap::new();

    for line in workflows.lines() {
        let (name, conditions) = line.split_once('{').unwrap();
        let (conditions, status) = conditions.split_once('}').unwrap();
        let conditions: Vec<_> = conditions.split(',').collect();
        let mut conditions: HashMap<_, _> = conditions
            .iter()
            .map(|condition| {
                let (category, range) = condition.split_once(':').unwrap();
                let (min, max) = range.split_once('-').unwrap();
                let min = min.parse::<usize>().unwrap();
                let max = max.parse::<usize>().unwrap();
                (
                    Category::from_char(category.chars().next().unwrap()),
                    (min, max),
                )
            })
            .collect();
        let status = Status::from_char(status.chars().next().unwrap());
        workflow_map.insert(name, (conditions, status));
    }

    println!("workflow_map: {:?}", workflow_map);
    0
}
