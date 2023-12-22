use std::{cmp, collections::HashMap, time::Instant};

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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    status: Option<Status>,
}

impl Part {
    fn from_line(line: &str) -> Self {
        let mut part = Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
            status: None,
        };

        let line = line.replace(['{', '}'], "");
        line.split(',').for_each(|p| {
            let (cat, val) = p.split_once('=').unwrap();
            let cat = Category::from_char(cat.chars().next().unwrap());
            let val = val.parse::<usize>().unwrap();

            match cat {
                Category::ExtremelyCoolLooking => part.x = val,
                Category::Musical => part.m = val,
                Category::Aerodynamic => part.a = val,
                Category::Shiny => part.s = val,
            }
        });

        part
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Rule {
    operation: char,
    category: char,
    size: usize,
}

fn main() {
    println!("Part 1 = {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let start = Instant::now();
    let mut workflow_map: HashMap<String, Vec<(_, _)>> = HashMap::new();
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    workflows.lines().for_each(|l| {
        let w: String = l.replace(['{', '}'], " ");
        let (name, w) = w.trim_end().split_once(' ').unwrap();
        let w: Vec<String> = w.split(',').map(String::from).collect();
        let w = w
            .iter()
            .map(|i| {
                let split = i.split(':').collect::<Vec<_>>();
                match split.len() {
                    1 => (None, split[0].to_string()),
                    2 => {
                        let mut op_split = split[0].chars();
                        let category = op_split.next().unwrap();
                        let operation = op_split.next().unwrap();
                        let size = op_split.collect::<String>().parse().unwrap();
                        let dest = split[1].to_string();
                        (
                            Some(Rule {
                                category,
                                operation,
                                size,
                            }),
                            dest,
                        )
                    }
                    _ => unreachable!("should have at most 2 parts"),
                }
            })
            .collect();

        workflow_map.insert(name.to_string(), w);
    });
    let mut parts = parts.lines().map(Part::from_line).collect::<Vec<_>>();

    let mut total = 0;
    for part in &mut parts {
        let mut workflow = workflow_map.get("in").unwrap();

        while part.status.is_none() {
            for (rule, dest) in workflow {
                // if there is no rule we are directly at a destination
                if rule.is_none() {
                    match dest.as_str() {
                        "A" => {
                            part.status = Some(Status::Accepted);
                            break;
                        }
                        "R" => {
                            part.status = Some(Status::Rejected);
                            break;
                        }
                        _ => {
                            workflow = workflow_map.get(dest).unwrap();
                            break;
                        }
                    }
                } else {
                    let rule = rule.as_ref().unwrap();
                    let value = match rule.category {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => unreachable!("Invalid category"),
                    };
                    match rule.operation {
                        '<' => {
                            if value < rule.size {
                                match dest.as_str() {
                                    "A" => {
                                        part.status = Some(Status::Accepted);
                                        break;
                                    }
                                    "R" => {
                                        part.status = Some(Status::Rejected);
                                        break;
                                    }
                                    _ => {
                                        workflow = workflow_map.get(dest).unwrap();
                                        break;
                                    }
                                }
                            } else {
                                continue;
                            }
                        }
                        '>' => {
                            if value > rule.size {
                                match dest.as_str() {
                                    "A" => {
                                        part.status = Some(Status::Accepted);
                                        break;
                                    }
                                    "R" => {
                                        part.status = Some(Status::Rejected);
                                        break;
                                    }
                                    _ => {
                                        workflow = workflow_map.get(dest).unwrap();
                                        break;
                                    }
                                }
                            } else {
                                continue;
                            }
                        }
                        _ => unreachable!("Invalid operation"),
                    }
                }
            }
        }

        if part.status == Some(Status::Accepted) {
            total += part.value();
        }
    }

    let end = Instant::now();
    println!("Part 1 in {:?}", end.duration_since(start));

    total
}
