// cargo run --example day7 -- (part1|part2) (example_input|final_input)

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
#[structopt(name = "day7", about = "Advent of Code 2021 Day 7")]
pub struct Args {
    part: Part,
    input: Input,
}

fn solve(file_path: &str, part: Part) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let initial_state: Vec<i32> = contents.split(',').map(|x| x.parse().unwrap()).collect();

    let max = *initial_state.iter().max().unwrap();
    let min = *initial_state.iter().min().unwrap();

    let mut best_fuel: Option<i32> = None;
    let mut best_option: Option<i32> = None;
    for option in min..=max {
        let mut sum_option = 0;
        for crab in initial_state.iter() {
            let distance = (*crab - option).abs();
            let fuel = match part {
                Part::Part1 => distance,
                Part::Part2 => (distance * (distance + 1)) / 2,
            };
            sum_option += fuel;
        }
        match best_fuel {
            None => { best_fuel = Some(sum_option); best_option = Some(option) },
            Some(current) => if current > sum_option { best_fuel = Some(sum_option); best_option = Some(option); },
        }
        
    }
    println!("{:?} {:?}", best_option, best_fuel);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day7/example_input",
        Input::FinalInput => "data/day7/input",
    };
    solve(file_path, args.part)
}