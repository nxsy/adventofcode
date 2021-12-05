use std::{io::BufRead, str::FromStr};

use anyhow::{Result, Context};
use structopt::StructOpt;

#[derive(Default, Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
}

#[derive(Default, Debug)]
struct Transform {
    position: Position,
    aim: i32,
}


#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Command {
    Forward,
    Up,
    Down,
}

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
#[structopt(name = "day2", about = "Advent of Code 2021 Day 2")]
pub struct Args {
    part: Part,
    input: Input,
}

fn part1(file_path: &str) -> Result<()> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut position = Position::default();
    for line in reader.lines() {
        let line = line?;
        let (command, val) = line.split_once(" ").context("Unable to parse command")?;
        let val = val.parse::<i32>()?;
        let command = Command::from_str(command)?;
        match command {
            Command::Forward => position.horizontal += val,
            Command::Up => position.depth -= val,
            Command::Down => position.depth += val,
        }
    }
    println!("Final position: {:#?}", position);
    println!("Multiplied: {}", position.horizontal * position.depth);
    Ok(())
}

fn part2(file_path: &str) -> Result<()> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut transform = Transform::default();
    for line in reader.lines() {
        let line = line?;
        let (command, val) = line.split_once(" ").context("Unable to parse command")?;
        let val = val.parse::<i32>()?;
        let command = Command::from_str(command)?;
        match command {
            Command::Forward => {
                transform.position.horizontal += val;
                transform.position.depth += val * transform.aim;

            }
            Command::Up => transform.aim -= val,
            Command::Down => transform.aim += val,
        }
    }
    println!("Final position: {:#?}", transform);
    println!("Multiplied: {}", transform.position.horizontal * transform.position.depth);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day2/example_input",
        Input::FinalInput => "data/day2/input",
    };
    match args.part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}