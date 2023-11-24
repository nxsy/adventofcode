use std::fs::File;
use std::io::BufRead;

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
#[structopt(name = "day2", about = "Advent of Code 2021 Day 1")]
pub struct Args {
    part: Part,
    input: Input,
}

#[allow(dead_code)]
fn part1(file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut previous: Option<u32> = None;
    let mut increased_measurements = 0u32;
    for line in reader.lines() {
        let line = line?;
        let current: u32 = line.parse()?;
        if let Some(previous) = previous {
            if current > previous {
                increased_measurements += 1;
            }
        }
        previous = Some(current);
    }
    println!("Increased measurements: {}", increased_measurements);
    Ok(())
}

fn part2(file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let mut windows: Vec<u32> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let current: u32 = line.parse()?;
        windows.push(current);
        if i > 0 {
            windows[i - 1] += current;
        }
        if i > 1 {
            windows[i - 2] += current;
        }
    }

    let mut increased_measurements = 0u32;
    let mut previous = 0u32;
    for (i, v) in windows[0..(windows.len() - 2)].iter().enumerate() {
        println!("Window {}, value {}", i, v);
        if i > 0 && *v > previous {
            increased_measurements += 1;
        }
        previous = *v;
    }
    println!("Increased measurements: {}", increased_measurements);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day1/example_input",
        Input::FinalInput => "data/day1/input",
    };
    match args.part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}