//! Day 01
//!
//! Mostly getting the basic setup right (stupid stuff, like updating from
//! from structopt to clap), and then lamenting that `BinaryHeap`'s obviously
//! useful method for this (besides just calling `pop()` 3 times),
//! `into_iter_sorted`, is an unstable feature.

use std::collections::BinaryHeap;

use anyhow::Result;
use clap::Parser;

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

fn part1(file_data: &str) -> Result<()> {
    let mut lines = file_data.lines();
    let mut most_calories = 0;
    loop {
        let mut calories = 0;
        let should_continue = loop {
            match lines.next() {
                None => break false,
                Some(l) if l.is_empty() => break true,
                Some(l) => {
                    calories += l.parse::<i32>()?;
                }
            }
        };
        most_calories = most_calories.max(calories);
        if !should_continue {
            break;
        }
    }

    println!("{most_calories}");

    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let mut lines = file_data.lines();
    let mut most_calories_heap = BinaryHeap::new();
    loop {
        let mut calories = 0;
        let should_continue = loop {
            match lines.next() {
                None => break false,
                Some(l) if l.is_empty() => break true,
                Some(l) => {
                    calories += l.parse::<i32>()?;
                }
            }
        };
        most_calories_heap.push(calories);
        if !should_continue {
            break;
        }
    }

    println!(
        "{}",
        most_calories_heap
            .into_sorted_vec()
            .iter()
            .rev()
            .take(3)
            .sum::<i32>()
    );

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
