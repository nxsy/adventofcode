//! Day 01

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
    Example,
    Example2,
    Final,
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
    let sum = file_data.lines().map(line_to_number).sum::<Result<u32>>()?;
    println!("Sum: {}", sum);
    Ok(())
}

fn line_to_number(line: &str) -> Result<u32> {
    let numbers: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    if numbers.is_empty() {
        anyhow::bail!("No numbers found in line: {}", line);
    }
    let chars = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
    Ok(chars.parse()?)
}

fn part2(file_data: &str) -> Result<()> {
    let sum = file_data
        .lines()
        .map(line_to_number_part2)
        .sum::<Result<u32>>()?;
    println!("Sum: {}", sum);
    Ok(())
}

fn partial_line_to_number(s: &str) -> Option<u8> {
    let c = s.chars().next()?;
    if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else if s.starts_with("zero") {
        Some(0)
    } else {
        c.to_digit(10).map(|n| n as u8)
    }
}

fn line_to_number_part2(line: &str) -> Result<u32> {
    let numbers: Vec<u8> = (0..line.len())
        .filter_map(|i| partial_line_to_number(&line[i..]))
        .collect();

    if numbers.is_empty() {
        anyhow::bail!("No numbers found in line: {}", line);
    }

    let chars = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
    Ok(chars.parse()?)
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::Example => include_str!("example_input.txt"),
        Input::Example2 => include_str!("example_input2.txt"),
        Input::Final => include_str!("input.txt"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
