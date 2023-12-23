use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./input.txt");
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum PulseKind {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    FlipFlop,    // %
    Conjunction, // &
    Broadcast,   // broadcaster
}

#[derive(Debug, Clone)]
struct PulseCount {
    low: usize,
    high: usize,
}

#[derive(Debug, Clone)]
struct Module {
    kind: ModuleKind,
    // TODO: inputs can maybe just be the name and last pulse received
    inputs: Option<Vec<String>>,
    on: bool,
    outputs: Vec<String>,
    count: PulseCount,
    pulse: Option<PulseKind>,
}

impl Module {
    fn pulse(
        &mut self,
        received: Option<PulseKind>,
        modules: &HashMap<String, Module>,
    ) -> Option<PulseKind> {
        match received {
            Some(PulseKind::Low) => self.count.low += 1,
            Some(PulseKind::High) => self.count.high += 1,
            None => {}
        }
        match self.kind {
            ModuleKind::FlipFlop => match received {
                Some(PulseKind::Low) => {
                    let pulse = match self.on {
                        true => {
                            self.pulse = Some(PulseKind::Low);
                            Some(PulseKind::Low)
                        }
                        false => {
                            self.pulse = Some(PulseKind::High);
                            Some(PulseKind::High)
                        }
                    };
                    self.on = !self.on;
                    pulse
                }
                _ => {
                    self.pulse = None;
                    None
                }
            },
            ModuleKind::Conjunction => {
                if self
                    .inputs
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|i| modules.get(i).unwrap().pulse == Some(PulseKind::High))
                {
                    self.pulse = Some(PulseKind::Low);
                    Some(PulseKind::Low)
                } else {
                    self.pulse = Some(PulseKind::High);
                    return Some(PulseKind::High);
                }
            }
            ModuleKind::Broadcast => {
                self.pulse = Some(PulseKind::Low);
                Some(PulseKind::Low)
            }
        }
    }
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let mut temp_modules: HashMap<String, Module> = HashMap::new();
    input.lines().for_each(|l| {
        let mut module = Module {
            kind: ModuleKind::Broadcast,
            inputs: None,
            on: false,
            outputs: Vec::new(),
            count: PulseCount { low: 0, high: 0 },
            pulse: None,
        };
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
        module.kind = kind;
        module.outputs = outputs.split(", ").map(|s| s.to_string()).collect();
        temp_modules.insert(name, module);
    });

    let mut new_modules = temp_modules.clone();

    for (name, module) in temp_modules {
        for output in &module.outputs {
            let output_module = new_modules.get_mut(output).unwrap();
            match output_module.inputs {
                Some(ref mut inputs) => inputs.push(name.to_string()),
                None => output_module.inputs = Some(vec![name.to_string()]),
            }
        }
    }

    for (name, module) in &new_modules {
        println!("{}: {:?}", name, module);
    }

    let mut broadcast = new_modules.get("broadcaster").unwrap().clone();
    let pulse = broadcast.pulse(None, &new_modules);

    let mut states = VecDeque::new();

    for output in &broadcast.outputs {
        states.push_front((pulse.clone(), output.to_string()));
    }

    // TODO: refactor this to not use new modules mutably and immutably for pulse

    // while let Some((pulse, name)) = states.pop_back() {
    //     let mut module = new_modules.get_mut(&name).unwrap();
    //     let pulse = module.pulse(pulse, &new_modules);
    //     for output in &module.outputs {
    //         states.push_front((pulse.clone(), output.to_string()));
    //     }
    // }

    0
}
