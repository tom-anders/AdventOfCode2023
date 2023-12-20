use std::collections::{HashMap, VecDeque};

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
enum Pulse {
    #[display(fmt = "low")]
    Low,
    #[display(fmt = "high")]
    High,
}

#[derive(Debug, Clone, Copy)]
struct Event<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module<'a> {
    name: &'a str,
    outputs: Vec<&'a str>,
    ty: ModuleType<'a>,
    sent_pulse_this_iteration: Option<Pulse>,
}

impl<'a> Module<'a> {
    fn receive(
        &mut self,
        from: &'a str,
        input_pulse: Pulse,
    ) -> impl Iterator<Item = Event<'a>> + '_ {
        let output_pulse = match &mut self.ty {
            ModuleType::FlipFlop { on } => {
                if input_pulse == Pulse::Low {
                    *on = !*on;
                    if *on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                } else {
                    None
                }
            }
            ModuleType::Broadcaster => Some(input_pulse),
            ModuleType::Conjunction { memory } => {
                *memory.get_mut(from).unwrap() = input_pulse;
                Some(if let Ok(Pulse::High) = memory.values().all_equal_value() {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
        };
        if output_pulse.is_some() {
            self.sent_pulse_this_iteration = output_pulse;
        }

        let our_name = self.name;
        self.outputs.iter().flat_map(move |&output| {
            output_pulse.map(|pulse| Event { from: our_name, to: output, pulse })
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { on: bool },
    Conjunction { memory: HashMap<&'a str, Pulse> },
}

fn parse_module(s: &str) -> (&str, Module) {
    let (module, outputs) = s.split(" -> ").collect_tuple().unwrap();
    let outputs = outputs.split(", ");

    let (name, ty) = match (&module[..1], &module[1..]) {
        ("%", name) => (name, ModuleType::FlipFlop { on: false }),
        ("&", name) => (name, ModuleType::Conjunction { memory: HashMap::new() }),
        ("b", "roadcaster") => (module, ModuleType::Broadcaster),
        _ => unreachable!(),
    };

    (name, Module { name, outputs: outputs.collect(), ty, sent_pulse_this_iteration: None })
}

#[derive(Debug, Default, Clone)]
struct Network<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    event_queue: VecDeque<Event<'a>>,
    low_pulse_count: usize,
    high_pulse_count: usize,
}

impl<'a> Network<'a> {
    fn new(input: &'a Input) -> Self {
        let mut modules: HashMap<_, _> = input.lines().map(parse_module).collect();

        for (input, module) in modules.clone() {
            for output in module.outputs {
                if let Some(Module { ty: ModuleType::Conjunction { memory }, .. }) =
                    modules.get_mut(output)
                {
                    memory.insert(input, Pulse::Low);
                }
            }
        }

        Self { modules, ..Default::default() }
    }

    fn push_button(&mut self) {
        for module in self.modules.values_mut() {
            module.sent_pulse_this_iteration = None;
        }

        self.event_queue.push_back(Event { from: "button", to: "broadcaster", pulse: Pulse::Low });

        loop {
            match self.event_queue.pop_front() {
                None => break,
                Some(event) => {
                    let Event { from, to, pulse } = event;
                    match pulse {
                        Pulse::Low => self.low_pulse_count += 1,
                        Pulse::High => self.high_pulse_count += 1,
                    }

                    if let Some(module) = self.modules.get_mut(to) {
                        self.event_queue.extend(module.receive(from, pulse));
                    }
                }
            }
        }
    }

    fn find_high_cycle(mut self, tracked_output: &str) -> (usize, usize) {
        let mut cycle_start = None;
        let mut i = 1;

        loop {
            self.push_button();
            if matches!(self.modules[tracked_output].sent_pulse_this_iteration, Some(Pulse::High)) {
                match cycle_start {
                    Some(cycle_start) => return (cycle_start, i - cycle_start),
                    None => cycle_start = Some(i),
                }
            }

            i += 1;
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut network = Network::new(input);

    for _ in 0..1000 {
        network.push_button();
    }

    network.high_pulse_count * network.low_pulse_count
}

fn find_cycle_match(
    (start1, cycle1): (usize, usize),
    (start2, cycle2): (usize, usize),
) -> (usize, usize) {
    let mut i = start1;
    let mut j = start2;
    loop {
        if i == j {
            return (i, cycle1 * cycle2);
        }
        i += cycle1;
        j += cycle2;
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let network = Network::new(&input);

    let mut cycles = vec![];
    for conj in ["ms", "xd", "zt", "gt"] {
        let sub_cycles = network
            .modules
            .iter()
            .filter(|(name, module)| {
                matches!(module.ty, ModuleType::FlipFlop { .. })
                    && (module.outputs.contains(&conj)
                        || network.modules[conj].outputs.contains(name))
            })
            .map(|(name, _)| network.clone().find_high_cycle(name))
            .sorted()
            .collect_vec();
        cycles.push(*sub_cycles.last().unwrap());
    }

    (
        part1(&input),
        cycles
            .iter()
            .skip(1)
            .fold(*cycles.first().unwrap(), |lhs, rhs| find_cycle_match(lhs, *rhs))
            .1,
    )
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufWriter};

    use super::*;

    #[test]
    fn draw_graph() {
        let input = Input::new(&format!("{}/../inputs/20.in", std::env!("CARGO_MANIFEST_DIR")));
        let network = Network::new(&input);

        let mut output = BufWriter::new(
            File::create(format!("{}/../inputs/graph20.dot", std::env!("CARGO_MANIFEST_DIR")))
                .unwrap(),
        );
        use dot_writer::*;
        let mut writer = DotWriter::from(&mut output);

        writer.set_pretty_print(false);

        let mut graph = writer.digraph();
        for (name, module) in network.modules {
            graph
                .node_named(name)
                .set_color(match module.ty {
                    ModuleType::Broadcaster => Color::Grey,
                    ModuleType::FlipFlop { .. } => Color::Red,
                    ModuleType::Conjunction { .. } => Color::PaleTurquoise,
                })
                .set_style(Style::Filled);
            for output in module.outputs {
                graph.edge(name, output);
            }
        }
        graph.node_named("rx").set_color(Color::PaleGreen).set_style(Style::Filled);
    }
}
