use std::io::BufRead;
use std::fs::read_to_string;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", about = "Advent of Code 2021 Day 3")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug, Default)]
pub struct Bits {
    bits: Vec<Frequency>,
}

impl Bits {
    pub fn new(num_bits: u32) -> Self {
        let bits: Vec<Frequency> = vec![Frequency::default(); num_bits as usize];
        Self { bits }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Frequency {
    num_0: u32,
    num_1: u32,
}

fn part1(file_path: &str, input: Input) -> Result<()> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let num_bits = match input {
        Input::ExampleInput => 5,
        Input::FinalInput => 12,
    };
    let mut bits = Bits::new(num_bits);
    for line in reader.lines() {
        let line = line?;
        for (i, char) in line.chars().enumerate() {
            match char {
                '0' => bits.bits[i].num_0 += 1,
                '1' => bits.bits[i].num_1 += 1,
                _ => {
                    panic!("Unknown value")
                }
            }
        }
    }
    let result: Vec<_> = bits
        .bits
        .iter()
        .map(|f| if f.num_0 > f.num_1 { 0 } else { 1 })
        .collect();
    println!("Bits: {:#?}", bits);
    println!("Result: {:#?}", result);
    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for (i, v) in result.iter().rev().enumerate() {
        gamma += 2u32.pow(i as u32) * *v;
        epsilon += 2u32.pow(i as u32) * (1 - *v);
    }
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Total: {}", gamma * epsilon);
    Ok(())
}

fn part2(file_path: &str, input: Input) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let num_bits = match input {
        Input::ExampleInput => 5,
        Input::FinalInput => 12,
    };
    let mut bits = Bits::new(num_bits);
    for line in contents.lines() {
        for (i, char) in line.chars().enumerate() {
            match char {
                '0' => bits.bits[i].num_0 += 1,
                '1' => bits.bits[i].num_1 += 1,
                _ => {
                    panic!("Unknown value")
                }
            }
        }
    }

    let oxygen_string = {
        let mut lines: Vec<_> = contents.lines().map(|x| x.to_owned()).collect();
        for i in 0..bits.bits.len() {
            let mut f = Frequency::default();
            for line in lines.iter() {
                let char = line.as_bytes()[i] as char;
                match char {
                    '0' => f.num_0 += 1,
                    '1' => f.num_1 += 1,
                    _ => {
                        panic!("Unknown value")
                    }
                }
            }
            
            let target = if f.num_0 > f.num_1 { "0" } else { "1" };
            println!("0s: {}, 1s: {}, target is {}", f.num_0, f.num_1, target);
            lines = lines.iter().filter(|x| x.as_bytes()[i] == target.as_bytes()[0]).map(|x| x.to_owned()).collect();
            println!("Lines remaining: {:?}", lines);
            if lines.len() == 1 {
                break;
            }
        }
        lines[0].to_owned()
    };
    println!("Oxygen: {:#?}", oxygen_string);

    let scrubber_string = {
        let mut lines: Vec<_> = contents.lines().map(|x| x.to_owned()).collect();
        for i in 0..bits.bits.len() {
            let mut f = Frequency::default();
            for line in lines.iter() {
                let char = line.as_bytes()[i] as char;
                match char {
                    '0' => f.num_0 += 1,
                    '1' => f.num_1 += 1,
                    _ => {
                        panic!("Unknown value")
                    }
                }
            }
            
            let target = if f.num_0 <= f.num_1 { "0" } else { "1" };
            println!("0s: {}, 1s: {}, target is {}", f.num_0, f.num_1, target);
            lines = lines.iter().filter(|x| x.as_bytes()[i] == target.as_bytes()[0]).map(|x| x.to_owned()).collect();
            println!("Lines remaining: {:?}", lines);
            if lines.len() == 1 {
                break;
            }
        }
        lines[0].to_owned()
    };
    println!("Scrubber: {:#?}", scrubber_string);

    let mut oxygen = 0u32;

    for (i, v) in oxygen_string.chars().rev().enumerate() {
        let v = match v {
            '0' => 0,
            '1' => 1,
            _ => {
                panic!("Unknown value")
            }
        };
        oxygen += 2u32.pow(i as u32) * v;
    }
    println!("Oxygen: {:#?}", oxygen);

    let mut scrubber = 0u32;

    for (i, v) in scrubber_string.chars().rev().enumerate() {
        let v = match v {
            '0' => 0,
            '1' => 1,
            _ => {
                panic!("Unknown value")
            }
        };
        scrubber += 2u32.pow(i as u32) * v;
    }
    println!("Scrubber: {:#?}", scrubber);

    println!("Total: {}", scrubber * oxygen);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day3/example_input",
        Input::FinalInput => "data/day3/input",
    };
    match args.part {
        Part::Part1 => part1(file_path, args.input),
        Part::Part2 => part2(file_path, args.input),
    }
}
