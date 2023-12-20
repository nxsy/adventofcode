mod parsing;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use anyhow::Result;

use thiserror::Error;

use crate::parsing::parse;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Module {
    Button,
    Broadcaster,
    FlipFlop(&'static str),
    Conjunction(&'static str),
    Untyped(&'static str),
}

impl Module {
    fn name(&self) -> &'static str {
        match self {
            Module::Button => "button",
            Module::Broadcaster => "broadcaster",
            Module::FlipFlop(name) | Module::Conjunction(name) | Module::Untyped(name) => name,
        }
    }
}

#[derive(Debug)]
struct Config {
    modules: BTreeMap<Module, Vec<Module>>,
    module_name_to_module: BTreeMap<&'static str, Module>,
    reverse_modules: BTreeMap<Module, Vec<Module>>,
}

#[derive(Debug, Default)]
struct ModuleState {
    stack: VecDeque<(Module, Module, bool)>,

    values: BTreeMap<Module, bool>,
    last_pulse: BTreeMap<Module, bool>,

    watched: BTreeSet<Module>,
    previous: BTreeMap<Module, (u64, u64)>,
    button_press: u64,

    low_pulses: u64,
    high_pulses: u64,
    log: Vec<String>,
}

impl ModuleState {
    fn log_pulse(&mut self, from_module: Module, to_module: Module, pulse: bool) {
        if pulse {
            self.high_pulses += 1;
        } else {
            self.low_pulses += 1;
        }
        self.last_pulse.insert(from_module, pulse);
        self.log.push(format!(
            "{} -{}-> {}",
            from_module.name(),
            if pulse { "high" } else { "low" },
            to_module.name()
        ));

        if self.watched.contains(&to_module) && !pulse {
            let previous = self.previous.entry(to_module).or_default();
            *previous = (self.button_press, previous.0);
        }
    }

    fn flip_value(&mut self, module: Module) -> bool {
        let last_value = self.values.entry(module).or_default();
        *last_value = !*last_value;
        *last_value
    }

    fn push(&mut self, from_module: Module, to_modules: &[Module], pulse: bool) {
        for to_module in to_modules {
            self.stack.push_back((from_module, *to_module, pulse));
        }
    }

    fn lcm(&self) -> Option<u64> {
        if self.watched.is_empty() || self.previous.len() != self.watched.len() {
            return None;
        }
        if self.previous.iter().any(|(_, x)| x.1 == 0) {
            return None;
        }
        let lcm = self
            .previous
            .iter()
            .fold(1, |acc, (_, x)| num::integer::lcm(acc, x.0 - x.1));
        Some(lcm)
    }
}

fn solve1(
    config: &Config,
    num_buttons: u64,
    pause_on_rx: bool,
) -> Result<(u64, String, Option<u64>)> {
    let mut module_state = ModuleState::default();

    if pause_on_rx {
        let rx_parent = config.reverse_modules[&config.module_name_to_module["rx"]][0];
        module_state.watched = config.reverse_modules[&rx_parent].iter().cloned().collect();
    }

    for _ in 0..num_buttons {
        module_state.button_press += 1;
        module_state.stack = VecDeque::from([(Module::Button, Module::Button, false)]);

        while let Some((from_module, module, pulse)) = module_state.stack.pop_front() {
            solve1_once(config, &mut module_state, from_module, module, pulse)?;
        }

        if pause_on_rx && module_state.lcm().is_some() {
            break;
        }
    }

    Ok((
        module_state.low_pulses * module_state.high_pulses,
        module_state.log.join("\n"),
        module_state.lcm(),
    ))
}

fn solve1_once(
    config: &Config,
    module_state: &mut ModuleState,
    from_module: Module,
    module: Module,
    pulse: bool,
) -> Result<()> {
    if module != Module::Button {
        module_state.log_pulse(from_module, module, pulse);
    }

    match module {
        Module::Button | Module::Broadcaster => {
            module_state.push(module, &config.modules[&module], false);
        }
        Module::FlipFlop(_) => {
            if !pulse {
                let pulse_to_send = module_state.flip_value(module);

                module_state.push(module, &config.modules[&module], pulse_to_send);
            }
        }
        Module::Conjunction(_) => {
            let pulse_to_send = config.reverse_modules[&module]
                .iter()
                .any(|m| !module_state.last_pulse.get(m).copied().unwrap_or_default());
            module_state.push(module, &config.modules[&module], pulse_to_send);
        }
        Module::Untyped(_) => {}
    }
    Ok(())
}

fn part1(input: &'static str) -> Result<String> {
    let (_, config) = parse(input)?;
    let (score, _, _) = solve1(&config, 1000, false)?;
    Ok(score.to_string())
}

fn part2(input: &'static str) -> Result<String> {
    let (_, config) = parse(input)?;
    let (_, _, buttons_to_rx) = solve1(&config, 10000000000, true)?;
    Ok(buttons_to_rx.unwrap().to_string())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let part1_result = match part1(input) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part1: {}", part1_result);
    let part2_result = match part2(input) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "32000000";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_solve1() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let (_, config) = parse(file_data)?;
        let expected = include_str!("example_output.txt");
        let (score, actual, _) = solve1(&config, 1, false)?;
        assert_eq!(actual, expected);
        assert_eq!(score, 32);
        Ok(())
    }

    #[test]
    fn test_solve1_ex1_1000() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let (_, config) = parse(file_data)?;
        let (score, _, _) = solve1(&config, 1000, false)?;
        assert_eq!(score, 32000000);
        Ok(())
    }

    #[test]
    fn test_solve1_ex2() -> Result<()> {
        let file_data = include_str!("example_input2.txt");
        let (_, config) = parse(file_data)?;

        let cases = [
            (1, include_str!("example_output2_1.txt")),
            (2, include_str!("example_output2_2.txt")),
            (3, include_str!("example_output2_3.txt")),
            (4, include_str!("example_output2_4.txt")),
        ];

        for (num_buttons, expected) in cases {
            let (_, actual, _) = solve1(&config, num_buttons, false)?;
            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_solve1_ex2_1000() -> Result<()> {
        let file_data = include_str!("example_input2.txt");
        let (_, config) = parse(file_data)?;
        let (score, _, _) = solve1(&config, 1000, false)?;
        assert_eq!(score, 11687500);
        Ok(())
    }
}
