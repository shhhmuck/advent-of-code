use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("./input.txt");
#[allow(dead_code)]
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
        line.replace(['{', '}'], "").split(',').for_each(|p| {
            let (category, val) = p.split_once('=').unwrap();
            let category = category.chars().next().unwrap();
            let val = val.parse::<usize>().unwrap();
            match category {
                'x' => part.x = val,
                'm' => part.m = val,
                'a' => part.a = val,
                's' => part.s = val,
                _ => unreachable!("Invalid category"),
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
    // let s = Instant::now();
    // println!(
    //     "Part 1 = {} in {:?}",
    //     part_1(INPUT),
    //     Instant::now().duration_since(s)
    // );
    let s = Instant::now();
    println!(
        "Part 2 = {} in {:?}",
        part_2(INPUT),
        Instant::now().duration_since(s)
    );
}

fn part_1(input: &str) -> usize {
    let mut workflow_map: HashMap<String, Vec<(_, _)>> = HashMap::new();
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    for line in workflows.lines() {
        let line: String = line.replace(['{', '}'], " ");
        let (name, rules) = line.trim_end().split_once(' ').unwrap();
        let rules: Vec<String> = rules.split(',').map(String::from).collect();
        let rules: Vec<(Option<Rule>, String)> = rules
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
        workflow_map.insert(name.to_string(), rules);
    }
    let mut parts = parts.lines().map(Part::from_line).collect::<Vec<_>>();

    let mut total = 0;
    for part in &mut parts {
        // start workflow at in
        let mut workflow = workflow_map.get("in").unwrap();
        while part.status.is_none() {
            for (rule, dest) in workflow {
                // No rule we are directly at a destination
                if rule.is_none() {
                    match dest.as_str() {
                        "A" => part.status = Some(Status::Accepted),
                        "R" => part.status = Some(Status::Rejected),
                        _ => workflow = workflow_map.get(dest).unwrap(),
                    }
                    break;
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
                                    "A" => part.status = Some(Status::Accepted),
                                    "R" => part.status = Some(Status::Rejected),
                                    _ => workflow = workflow_map.get(dest).unwrap(),
                                }
                                break;
                            } else {
                                continue;
                            }
                        }
                        '>' => {
                            if value > rule.size {
                                match dest.as_str() {
                                    "A" => part.status = Some(Status::Accepted),
                                    "R" => part.status = Some(Status::Rejected),
                                    _ => workflow = workflow_map.get(dest).unwrap(),
                                }
                                break;
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

    total
}

fn part_2(input: &str) -> usize {
    let mut workflow_map: HashMap<String, Vec<(_, _)>> = HashMap::new();
    let (workflows, _parts) = input.split_once("\n\n").unwrap();
    for line in workflows.lines() {
        let line: String = line.replace(['{', '}'], " ");
        let (name, rules) = line.trim_end().split_once(' ').unwrap();
        let rules: Vec<String> = rules.split(',').map(String::from).collect();
        let rules: Vec<(Option<Rule>, String)> = rules
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
        workflow_map.insert(name.to_string(), rules);
    }

    let mut accepted = vec![];
    let mut states = vec![("in", (1, 4000), (1, 4000), (1, 4000), (1, 4000))];

    while let Some((destination, mut m_range, mut a_range, mut s_range, mut x_range)) = states.pop()
    {
        if destination == "A" {
            accepted.push((m_range, a_range, s_range, x_range));
            continue;
        }
        if destination == "R" {
            continue;
        }
        let workflow = workflow_map.get(destination).unwrap();
        for (rule, dest) in workflow {
            if rule.is_none() {
                states.push((dest, m_range, a_range, s_range, x_range));
                continue;
            }
            let rule = rule.as_ref().unwrap();
            match rule.operation {
                '<' => {
                    match rule.category {
                        'm' => {
                            if m_range.0 < rule.size && m_range.1 < rule.size {
                                states.push((dest, m_range, a_range, s_range, x_range));
                            } else if m_range.0 < rule.size && m_range.1 > rule.size {
                                // split, add a state for within range to go to next dest
                                states.push((
                                    dest,
                                    (m_range.0, rule.size - 1),
                                    a_range,
                                    s_range,
                                    x_range,
                                ));
                                // shorten range and fall thru to next
                                m_range = (rule.size, m_range.1);
                            }
                            continue;
                        }
                        'a' => {
                            if a_range.0 < rule.size && a_range.1 < rule.size {
                                states.push((dest, m_range, a_range, s_range, x_range));
                            } else if a_range.0 < rule.size && a_range.1 > rule.size {
                                // split, add a state for within range to go to next dest
                                states.push((
                                    dest,
                                    m_range,
                                    (a_range.0, rule.size - 1),
                                    s_range,
                                    x_range,
                                ));
                                // shorten range and fall thru to next
                                a_range = (rule.size, a_range.1);
                            }
                            continue;
                        }
                        's' => {
                            if s_range.0 < rule.size && s_range.1 < rule.size {
                                states.push((dest, m_range, a_range, s_range, x_range));
                            } else if s_range.0 < rule.size && s_range.1 > rule.size {
                                // split, add a state for within range to go to next dest
                                let split = (s_range.0, rule.size - 1);
                                states.push((dest, m_range, a_range, split, x_range));
                                // shorten range and fall thru to next
                                s_range = (rule.size, s_range.1);
                            }
                            continue;
                        }
                        'x' => {
                            if x_range.0 < rule.size && x_range.1 < rule.size {
                                states.push((dest, m_range, a_range, s_range, x_range));
                            } else if x_range.0 < rule.size && x_range.1 > rule.size {
                                // split, add a state for within range to go to next dest
                                states.push((
                                    dest,
                                    m_range,
                                    a_range,
                                    s_range,
                                    (x_range.0, rule.size - 1),
                                ));
                                // shorten range and fall thru to next
                                x_range = (rule.size, x_range.1);
                            }
                            continue;
                        }
                        _ => unreachable!("Invalid category"),
                    }
                }
                '>' => match rule.category {
                    'm' => {
                        if m_range.0 > rule.size && m_range.1 > rule.size {
                            states.push((dest, m_range, a_range, s_range, x_range));
                        } else if m_range.0 < rule.size && m_range.1 > rule.size {
                            // split, add a state for within range to go to next dest
                            states.push((
                                dest,
                                (rule.size + 1, m_range.1),
                                a_range,
                                s_range,
                                x_range,
                            ));
                            // shorten range and fall thru to next
                            m_range = (m_range.0, rule.size);
                        }
                        continue;
                    }
                    'a' => {
                        if a_range.0 > rule.size && a_range.1 > rule.size {
                            states.push((dest, m_range, a_range, s_range, x_range));
                        } else if a_range.0 < rule.size && a_range.1 > rule.size {
                            // split, add a state for within range to go to next dest
                            states.push((
                                dest,
                                m_range,
                                (rule.size + 1, a_range.1),
                                s_range,
                                x_range,
                            ));
                            // shorten range and fall thru to next
                            a_range = (a_range.0, rule.size);
                        }
                        continue;
                    }
                    's' => {
                        if s_range.0 > rule.size && s_range.1 > rule.size {
                            states.push((dest, m_range, a_range, s_range, x_range));
                        } else if s_range.0 < rule.size && s_range.1 > rule.size {
                            // split, add a state for within range to go to next dest
                            states.push((
                                dest,
                                m_range,
                                a_range,
                                (rule.size + 1, s_range.1),
                                x_range,
                            ));
                            // shorten range and fall thru to next
                            s_range = (s_range.0, rule.size);
                        }
                        continue;
                    }
                    'x' => {
                        if x_range.0 > rule.size && x_range.1 > rule.size {
                            states.push((dest, m_range, a_range, s_range, x_range));
                        } else if x_range.0 < rule.size && x_range.1 > rule.size {
                            // split, add a state for within range to go to next dest
                            states.push((
                                dest,
                                m_range,
                                a_range,
                                s_range,
                                (rule.size + 1, x_range.1),
                            ));
                            // shorten range and fall thru to next
                            x_range = (x_range.0, rule.size);
                        }
                        continue;
                    }
                    _ => unreachable!("Invalid category"),
                },
                _ => unreachable!("Invalid operation"),
            }
        }
    }

    // for range in &accepted {
    //     println!("{:?}", range);
    // }

    let mut total = 0;

    for (m, a, s, x) in accepted {
        let range_total = (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1) * (x.1 - x.0 + 1);
        // println!("{}", range_total);
        total += range_total
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST), 19_114);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TEST), 167_409_079_868_000);
    }
}
