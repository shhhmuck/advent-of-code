use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::Instant;

use num::integer::lcm;

const INPUT: &str = include_str!("./input.txt");

trait Module: fmt::Debug {
    fn pulse(&mut self, received: (String, PulseKind)) -> Vec<(String, PulseKind)>;
    fn outputs(&self) -> Option<&[String]>;
    fn inputs(&mut self) -> Option<&mut Vec<(String, PulseKind)>>;
    fn counts(&self) -> (usize, usize);
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum PulseKind {
    #[default]
    Low,
    High,
}

#[derive(Debug, Clone, Default)]
enum ModuleKind {
    FlipFlop,    // %
    Conjunction, // &
    #[default]
    Broadcast, // broadcaster
}

#[derive(Debug, Clone, Default)]
struct PulseCount {
    low: usize,
    high: usize,
}

#[derive(Debug)]
struct Conjunction {
    inputs: Vec<(String, PulseKind)>,
    outputs: Vec<String>,
    count: PulseCount,
}

impl Module for Conjunction {
    fn pulse(&mut self, received: (String, PulseKind)) -> Vec<(String, PulseKind)> {
        match received.1 {
            PulseKind::Low => self.count.low += 1,
            PulseKind::High => self.count.high += 1,
        }

        let matched = self
            .inputs
            .iter_mut()
            .find(|input| input.0 == received.0)
            .unwrap();
        matched.1 = received.1;

        let kind = match self.inputs.iter().all(|(_, p)| p == &PulseKind::High) {
            true => Some(PulseKind::Low),
            false => Some(PulseKind::High),
        };

        self.outputs
            .iter()
            .map(|o| (o.clone(), kind.as_ref().unwrap().clone()))
            .collect()
    }
    fn outputs(&self) -> Option<&[String]> {
        Some(&self.outputs)
    }
    fn inputs(&mut self) -> Option<&mut Vec<(String, PulseKind)>> {
        Some(&mut self.inputs)
    }
    fn counts(&self) -> (usize, usize) {
        (self.count.low, self.count.high)
    }
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    count: PulseCount,
    outputs: Vec<String>,
}

impl Module for FlipFlop {
    fn pulse(&mut self, received: (String, PulseKind)) -> Vec<(String, PulseKind)> {
        match received.1 {
            PulseKind::Low => {
                self.count.low += 1;
                let pulse = match self.on {
                    true => Some(PulseKind::Low),
                    false => Some(PulseKind::High),
                };
                self.on = !self.on;
                self.outputs
                    .iter()
                    .map(|o| (o.clone(), pulse.as_ref().unwrap().clone()))
                    .collect()
            }
            PulseKind::High => {
                self.count.high += 1;
                vec![]
            }
        }
    }
    fn outputs(&self) -> Option<&[String]> {
        Some(&self.outputs)
    }
    fn inputs(&mut self) -> Option<&mut Vec<(String, PulseKind)>> {
        None
    }
    fn counts(&self) -> (usize, usize) {
        (self.count.low, self.count.high)
    }
}

#[derive(Debug)]
struct Broadcast {
    count: PulseCount,
    outputs: Vec<String>,
}

impl Module for Broadcast {
    fn pulse(&mut self, received: (String, PulseKind)) -> Vec<(String, PulseKind)> {
        match received.1 {
            PulseKind::Low => self.count.low += 1,
            PulseKind::High => self.count.high += 1,
        }
        self.outputs
            .iter()
            .map(|o| (o.clone(), PulseKind::Low))
            .collect()
    }
    fn outputs(&self) -> Option<&[String]> {
        Some(&self.outputs)
    }
    fn inputs(&mut self) -> Option<&mut Vec<(String, PulseKind)>> {
        None
    }
    fn counts(&self) -> (usize, usize) {
        (self.count.low, self.count.high)
    }
}

#[derive(Debug)]
struct End {
    count: PulseCount,
    inputs: Vec<(String, PulseKind)>,
}

impl Module for End {
    fn pulse(&mut self, received: (String, PulseKind)) -> Vec<(String, PulseKind)> {
        match received.1 {
            PulseKind::Low => self.count.low += 1,
            PulseKind::High => self.count.high += 1,
        }
        vec![]
    }
    fn outputs(&self) -> Option<&[String]> {
        None
    }
    fn inputs(&mut self) -> Option<&mut Vec<(String, PulseKind)>> {
        Some(&mut self.inputs)
    }
    fn counts(&self) -> (usize, usize) {
        (self.count.low, self.count.high)
    }
}

fn main() {
    let s = Instant::now();
    println!(
        "Part 1: {} in {:?}",
        part_1(INPUT),
        Instant::now().duration_since(s)
    );
    let s = Instant::now();
    println!(
        "Part 2: {} in {:?}",
        part_2(INPUT),
        Instant::now().duration_since(s)
    );
}

fn part_2(input: &str) -> usize {
    let mut in_out = vec![];
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    input.lines().for_each(|l| {
        let (input, outputs) = l.split_once(" -> ").unwrap();
        let chars = input.chars().collect::<Vec<char>>();
        let (kind, name) = match chars[0] {
            '%' => (ModuleKind::FlipFlop, chars[1..].iter().collect::<String>()),
            '&' => (
                ModuleKind::Conjunction,
                chars[1..].iter().collect::<String>(),
            ),
            _ => (ModuleKind::Broadcast, input.to_string()),
        };
        let outputs: Vec<String> = outputs.split(", ").map(String::from).collect();

        let module: Box<dyn Module> = match kind {
            ModuleKind::FlipFlop => Box::new(FlipFlop {
                on: false,
                count: PulseCount::default(),
                outputs,
            }),
            ModuleKind::Conjunction => Box::new(Conjunction {
                inputs: Vec::new(),
                outputs,
                count: PulseCount::default(),
            }),
            ModuleKind::Broadcast => Box::new(Broadcast {
                count: PulseCount::default(),
                outputs,
            }),
        };

        in_out.push((name.clone(), module.outputs().unwrap().to_vec()));
        modules.insert(name, module);
    });

    for (name, outputs) in in_out {
        for output in outputs {
            let output = modules.entry(output).or_insert(Box::new(End {
                count: PulseCount::default(),
                inputs: Vec::new(),
            }));
            if output.inputs().is_none() {
                continue;
            }
            output
                .inputs()
                .unwrap()
                .push((name.clone(), PulseKind::Low));
        }
    }

    let mut cycles: Vec<Vec<usize>> = vec![vec![]; 4];

    let mut idx = 0;
    loop {
        if cycles.iter().all(|c| c.len() >= 2) {
            // we have enough to calculate the lcm
            break;
        }

        let broadcast = modules.get_mut("broadcaster").unwrap();
        let mut states = VecDeque::new();

        broadcast
            .pulse((String::from("button"), PulseKind::Low))
            .iter()
            .for_each(|(name, kind)| {
                states.push_front(("broadcaster".to_string(), name.clone(), kind.clone()));
            });

        while let Some((from, to, kind)) = states.pop_back() {
            // dr is the only input to rx, and its a conjunction with 4 inputs
            // all need to be high to pulse low so lets find their cycles
            if to == "dr" && kind == PulseKind::High {
                match from.as_str() {
                    "mp" => cycles[0].push(idx),
                    "qt" => cycles[1].push(idx),
                    "qb" => cycles[2].push(idx),
                    "ng" => cycles[3].push(idx),
                    _ => unreachable!(),
                }
            }

            let module = modules.get_mut(&to).unwrap();
            let outputs = module.pulse((from.clone(), kind.clone()));
            outputs.iter().for_each(|output| {
                // println!("Adding state: {}: {:?}", output.0, output.1);
                states.push_front((to.clone(), output.0.clone(), output.1.clone()));
            });
        }

        idx += 1;
    }

    let mut calcs = vec![];

    // calc the cycle length which we can use for lcm
    for cycle in cycles {
        calcs.push(cycle[1] - cycle[0]);
    }

    calcs.iter().fold(1, |acc, x| lcm(acc, *x))
}

fn part_1(input: &str) -> usize {
    let mut in_out = vec![];
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    input.lines().for_each(|l| {
        let (input, outputs) = l.split_once(" -> ").unwrap();
        let chars = input.chars().collect::<Vec<char>>();
        let (kind, name) = match chars[0] {
            '%' => (ModuleKind::FlipFlop, chars[1..].iter().collect::<String>()),
            '&' => (
                ModuleKind::Conjunction,
                chars[1..].iter().collect::<String>(),
            ),
            _ => (ModuleKind::Broadcast, input.to_string()),
        };
        let outputs: Vec<String> = outputs.split(", ").map(String::from).collect();

        let module: Box<dyn Module> = match kind {
            ModuleKind::FlipFlop => Box::new(FlipFlop {
                on: false,
                count: PulseCount::default(),
                outputs,
            }),
            ModuleKind::Conjunction => Box::new(Conjunction {
                inputs: Vec::new(),
                outputs,
                count: PulseCount::default(),
            }),
            ModuleKind::Broadcast => Box::new(Broadcast {
                count: PulseCount::default(),
                outputs,
            }),
        };

        in_out.push((name.clone(), module.outputs().unwrap().to_vec()));
        modules.insert(name, module);
    });

    for (name, outputs) in in_out {
        for output in outputs {
            let output = modules.entry(output).or_insert(Box::new(End {
                count: PulseCount::default(),
                inputs: Vec::new(),
            }));
            if output.inputs().is_none() {
                continue;
            }
            output
                .inputs()
                .unwrap()
                .push((name.clone(), PulseKind::Low));
        }
    }

    for _ in 0..1000 {
        let broadcast = modules.get_mut("broadcaster").unwrap();
        let mut states = VecDeque::new();

        broadcast
            .pulse((String::from("button"), PulseKind::Low))
            .iter()
            .for_each(|(name, kind)| {
                states.push_front(("broadcaster".to_string(), name.clone(), kind.clone()));
            });

        while let Some((from, to, kind)) = states.pop_back() {
            // println!("Processing state: FROM: {} TO: {} KIND: {:?}", from, to, kind);
            let module = modules.get_mut(&to).unwrap();
            let outputs = module.pulse((from.clone(), kind.clone()));
            outputs.iter().for_each(|output| {
                // println!("Adding state: {}: {:?}", output.0, output.1);
                states.push_front((to.clone(), output.0.clone(), output.1.clone()));
            });
        }
    }

    let mut totals = (0_usize, 0_usize);
    for module in modules.values() {
        let counts = module.counts();
        totals.0 += counts.0;
        totals.1 += counts.1;
    }

    totals.0 * totals.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part_1_first() {
        assert_eq!(part_1(TEST1), 32000000);
    }

    #[test]
    fn test_part_1_second() {
        assert_eq!(part_1(TEST2), 11687500);
    }
}
