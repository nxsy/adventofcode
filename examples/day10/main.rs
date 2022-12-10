//! Day 10
//!
//! This was also fun.

use std::str::FromStr;

use adventofcode2022::prelude::*;

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    part: Part,
    #[arg(long)]
    input: Input,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum_macros::EnumDiscriminants)]
#[strum_discriminants(derive(strum_macros::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab_case"))]
enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug)]
struct Cpu {
    val: i32,
    prev_val: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            val: 1,
            prev_val: 1,
            cycle: 0,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        self.cycle += match InstructionDiscriminants::from(instruction) {
            InstructionDiscriminants::Addx => 2,
            InstructionDiscriminants::Noop => 1,
        };
        self.prev_val = self.val;
        match instruction {
            Instruction::Addx(v) => self.val += v,
            Instruction::Noop => {}
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;
    fn try_from(line: &str) -> Result<Self, anyhow::Error> {
        let (a, b) = if let Some((a, b)) = line.split_once(' ') {
            (a, Some(b))
        } else {
            (line, None)
        };
        let instruction = match (InstructionDiscriminants::from_str(a)?, b) {
            (InstructionDiscriminants::Addx, Some(b)) => Instruction::Addx(b.parse()?),
            (InstructionDiscriminants::Noop, None) => Instruction::Noop,
            _ => unimplemented!(),
        };
        Ok(instruction)
    }
}

fn part1(file_data: &str) -> Result<()> {
    let mut cpu = Cpu::new();
    let mut next_signal_cycle = 20;
    let mut signal_strength_sum = 0;
    for line in file_data.lines() {
        let instruction = Instruction::try_from(line)?;
        cpu.execute(instruction);
        if cpu.cycle >= next_signal_cycle {
            let signal_strength = (next_signal_cycle as i32) * cpu.prev_val;
            next_signal_cycle += 40;
            signal_strength_sum += signal_strength;
        }
    }
    dbg!(signal_strength_sum);
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let mut cpu = Cpu::new();
    let mut crt = HashSet::new();
    for line in file_data.lines() {
        let instruction = Instruction::try_from(line)?;
        let cycle = cpu.cycle;
        let val = cpu.val;
        cpu.execute(instruction);
        for c in cycle..cpu.cycle {
            let sprite_range = val - 1..=val + 1;
            let cm = (c % 40) as i32;
            if sprite_range.contains(&cm) {
                crt.insert(c);
            }
        }
    }
    for c in 0..cpu.cycle {
        if c % 40 == 0 {
            println!();
        }
        print!("{}", if crt.contains(&c) { "#" } else { "." });
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::ExampleInput => include_str!("example_input"),
        Input::FinalInput => include_str!("input"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
