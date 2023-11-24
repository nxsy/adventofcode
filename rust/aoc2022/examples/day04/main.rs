//! Day 04
//!
//!

use std::ops::Range;

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

struct RangePair(Range<u32>, Range<u32>);

impl From<&str> for RangePair {
    fn from(value: &str) -> Self {
        let [a, b]: [Range<u32>; 2] = value
            .split(',')
            .map(|x| {
                let [start, end]: [u32; 2] = x
                    .split('-')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                Range { start, end }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        RangePair(a, b)
    }
}

fn part1(file_data: &str) -> Result<()> {
    let mut count = 0;
    for line in file_data.lines() {
        let rp = RangePair::from(line);
        let (a, b) = { (rp.0, rp.1) };
        let contained =
            (a.start <= b.start && a.end >= b.end) || (a.start >= b.start && a.end <= b.end);
        // println!("{a:?} {b:?} {contained}");
        if contained {
            count += 1;
        }
    }
    println!("{count}");
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let mut count = 0;
    for line in file_data.lines() {
        let rp = RangePair::from(line);
        let (a, b) = { (rp.0, rp.1) };
        let overlap = !((a.end < b.start) || (a.start > b.end));
        // println!("{a:?} {b:?} {overlap}");
        if overlap {
            count += 1;
        }
    }
    println!("{count}");
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
