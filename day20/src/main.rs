use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Broadcaster,
    Output(Option<Pulse>),
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
}

impl Operation {
    fn state(&self) -> Option<Pulse> {
        match self {
            Operation::Broadcaster => None,
            Operation::Output(state) => *state,
            Operation::FlipFlop(state) => Some(*state),
            Operation::Conjunction(inputs) => {
                if inputs.iter().all(|(_, val)| *val == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }

    fn receive_pulse(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Operation::Broadcaster => Some(pulse),
            Operation::Output(_) => None,
            Operation::FlipFlop(state) => {
                if pulse == Pulse::Low {
                    *state = if *state == Pulse::High {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    self.state()
                } else {
                    None
                }
            }
            Operation::Conjunction(inputs) => {
                *inputs.get_mut(input).unwrap() = pulse;
                self.state()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    operation: Operation,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct Machine {
    modules: HashMap<String, Module>,
    low_pulses: u64,
    high_pulses: u64,
    button_presses: u64,
}

impl Machine {
    fn push_button(&mut self) {
        use std::collections::VecDeque;

        self.button_presses += 1;

        let mut queue = VecDeque::new();
        queue.push_back((
            String::from("button"),
            String::from("broadcaster"),
            Pulse::Low,
        ));
        while let Some((input_name, current_name, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => self.high_pulses += 1,
                Pulse::Low => self.low_pulses += 1,
            }
            if let Some(module) = self.modules.get_mut(&current_name) {
                if let Some(output_pulse) = module.operation.receive_pulse(&input_name, pulse) {
                    for output_name in module.outputs.iter() {
                        queue.push_back((current_name.clone(), output_name.clone(), output_pulse))
                    }
                }
            }
        }
    }

    fn all_lows(&self) -> bool {
        self.modules
            .iter()
            .all(|(_, module)| match module.operation.state() {
                Some(pulse) => pulse == Pulse::High,
                None => true,
            })
    }

    // fn find_node_inputs(&self, node: &str) -> Vec<String> {
    //     self.modules
    //         .iter()
    //         .filter(|(_, module)| module.outputs.iter().find(|x| &x[..] == node).is_some())
    //         .map(|(name, _)| name.clone())
    //         .collect()
    // }
}

#[derive(Debug, Clone)]
struct Input {
    machine: Machine,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let mut modules = HashMap::new();
        let mut connections = HashMap::new();
        for line in BufReader::new(file).lines().map(Result::unwrap) {
            let (module_type, outputs) = line.split_once(" -> ").unwrap();

            let (module_name, operation) = {
                if module_type == "broadcaster" {
                    (String::from(module_type), Operation::Broadcaster)
                } else if module_type.starts_with("%") {
                    (
                        String::from(&module_type[1..module_type.len()]),
                        Operation::FlipFlop(Pulse::Low),
                    )
                } else if module_type.starts_with("&") {
                    (
                        String::from(&module_type[1..module_type.len()]),
                        Operation::Conjunction(HashMap::new()),
                    )
                } else {
                    panic!("Invalid module")
                }
            };

            let outputs: Vec<String> = outputs.split(", ").map(String::from).collect();

            connections.insert(module_name.clone(), outputs.clone());
            modules.insert(module_name, Module { operation, outputs });
        }

        for (input, outputs) in connections.iter() {
            for output in outputs.iter() {
                if let Some(output_module) = modules.get_mut(output) {
                    if let Operation::Conjunction(conj_inputs) = &mut output_module.operation {
                        conj_inputs.insert(input.clone(), Pulse::Low);
                    }
                } else {
                    modules.insert(
                        output.clone(),
                        Module {
                            operation: Operation::Output(None),
                            outputs: Vec::new(),
                        },
                    );
                    if let Operation::Conjunction(conj_inputs) =
                        &mut modules.get_mut(output).unwrap().operation
                    {
                        conj_inputs.insert(input.clone(), Pulse::Low);
                    }
                }
            }
        }

        Self {
            machine: Machine {
                modules,
                low_pulses: 0,
                high_pulses: 0,
                button_presses: 0,
            },
        }
    }
}

fn part_1(input: &Input) -> u64 {
    let max_button_presses = 1000;
    let mut machine = input.machine.clone();

    while machine.button_presses < max_button_presses {
        machine.push_button();
        if machine.all_lows() {
            break;
        }
    }

    let loop_repeats = max_button_presses / machine.button_presses;
    let loop_low_pulses = machine.low_pulses * loop_repeats;
    let loop_high_pulses = machine.high_pulses * loop_repeats;

    machine.low_pulses = 0;
    machine.high_pulses = 0;
    for _ in 0..max_button_presses % machine.button_presses {
        machine.push_button();
    }

    (loop_low_pulses + machine.low_pulses) * (loop_high_pulses + machine.high_pulses)
}

fn part_2(_input: &Input) -> u64 {
    0
    // let mut machine = input.machine.clone();

    // let rx_inputs = machine.find_node_inputs("rx");
    // if rx_inputs.len() != 1 {
    //     return 0;
    // }

    // let rx_inputs = machine.find_node_inputs(&rx_inputs[0]);
    // let mut cycles = vec![None; rx_inputs.len()];

    // while cycles.iter().any(Option::is_none) {
    //     machine.push_button();
    //     for (i, input) in rx_inputs.iter().enumerate() {
    //         if let Some(pulse) = machine.modules.get(input).unwrap().operation.state() {
    //             if pulse == Pulse::High {
    //                 cycles[i] = Some(machine.button_presses);
    //             }
    //         }
    //     }
    // }

    // cycles.iter().map(|x| x.unwrap()).fold(1, |acc, x| acc * x)
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day20/sample_input_1.txt", PrintMode::None),
            ("day20/sample_input_2.txt", PrintMode::None),
            ("day20/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
