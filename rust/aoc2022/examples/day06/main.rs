//! Day 06
//!
//!

use adventofcode2022::prelude::*;

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
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

fn first_unique(line: &str, num_unique: usize) -> (usize, Vec<char>) {
    for (i, w) in line.chars().collect::<Vec<_>>().windows(num_unique).enumerate() {
        let s: HashSet<char> = HashSet::from_iter(w.iter().copied());
        if s.len() == num_unique {
            return (i + num_unique, Vec::from(w));
        }   
    }
    unreachable!()
}

fn part1(file_data: &str) -> Result<()> {
    for line in file_data.lines() {
        let (pos, chars) = first_unique(line, 4);
        println!("{pos} {chars:?}");
    }
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    for line in file_data.lines() {
        let (pos, chars) = first_unique(line, 14);
        println!("{pos} {chars:?}");
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
