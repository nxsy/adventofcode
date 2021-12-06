// cargo run --example day6 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
use std::fs::read_to_string;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString, PartialEq, Eq, PartialOrd, Ord)]
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
#[structopt(name = "day6", about = "Advent of Code 2021 Day 6")]
pub struct Args {
    part: Part,
    input: Input,
}

fn part1(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let initial_state: Vec<i32> = contents.split(',').map(|x| x.parse().unwrap()).collect();

    let mut state: Vec<i32> = initial_state;
    for _ in 0..256 {
        let mut new: Vec<i32> = Vec::new();
        for i in state.iter_mut() {
            *i -= 1;
            if *i == -1 {
                *i = 6;
                new.push(8);
            }
        }
        state.append(&mut new);
    }

    println!("Length is now: {}", state.len());

    Ok(())
}

fn part2(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let initial_state: Vec<i32> = contents
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut num_fish_by_timer: HashMap<i32, i64> = HashMap::new();
    for i in initial_state {
        num_fish_by_timer
            .entry(i)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for _ in 0..256 {
        let mut new_state: HashMap<i32, i64> = HashMap::new();
        for (k, v) in num_fish_by_timer {
            if k == 0 {
                *new_state.entry(6).or_default() += v;
                *new_state.entry(8).or_default() += v;
            } else {
                *new_state.entry(k - 1).or_default() += v;
            }
        }
        num_fish_by_timer = new_state;
    }

    println!("Length is now: {}", num_fish_by_timer.values().sum::<i64>());

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day6/example_input",
        Input::FinalInput => "data/day6/input",
    };
    match args.part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}
