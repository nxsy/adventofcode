// cargo run --example day5 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
use std::fs::read_to_string;

use anyhow::{Result, Context};
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
#[structopt(name = "day5", about = "Advent of Code 2021 Day 5")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug, PartialEq)]
struct Position(i32, i32);
impl Position {
    fn from_string(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(",").context("Couldn't parse coord")?;
        Ok(Self(x.parse::<i32>()?, y.parse::<i32>()?))
    }
}

fn part1(file_path: &str, part: Part) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();

    for line in contents.lines() {
        let (left, right) = line.split_once(" -> ").context("Couldn't parse line")?;
        let start = Position::from_string(left)?;
        let end = Position::from_string(right)?;

        let x_delta: i32 = (end.0 - start.0).signum();
        let y_delta: i32 = (end.1 - start.1).signum();

        if part == Part::Part1 && x_delta != 0 && y_delta != 0 {
            // Skip diagonals for part 1
            continue;
        }

        let mut current = start;
        loop {
            *grid.entry((current.0, current.1)).or_default() += 1;
            if current == end {
                break;
            }
            current = Position(current.0 + x_delta, current.1 + y_delta);
        }
    }

    let overlaps: i32 = grid.values().map(|x| -> i32 { (*x > 1) as i32 }).sum();
    println!("Overlaps: {}", overlaps);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day5/example_input",
        Input::FinalInput => "data/day5/input",
    };
    part1(file_path, args.part)
}
