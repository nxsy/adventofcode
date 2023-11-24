//! Day 25

use adventofcode2022::prelude::*;

use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
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

fn parse_line(input: &str) -> IResult<&str, i64> {
    let (input, chars) = not_line_ending(input)?;
    let mut r = 0;

    for (i, c) in chars.chars().rev().enumerate() {
        let m = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        };
        r += m * 5_i64.pow(i as u32);
    }
    Ok((input, r))
}

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(line_ending, parse_line)(input)
}

fn number_to_snafu(mut number: i64) -> String {
    
    let mut digits = Vec::new();
    for _ in 0.. {
        let mut carry = 0;
        let m = number % 5;
        digits.push(match m {
            m if m < 3 => m.to_string(),
            m if m == 3 => {
                carry = 1;
                "=".to_string()
            }
            m if m == 4 => {
                carry = 1;
                "-".to_string()
            }
            _ => unreachable!(),
        });
        number /= 5;
        number += carry;
        if number == 0 {
            break;
        }
    }
    digits.reverse();
    digits.into_iter().collect::<String>()
}

fn part1(file_data: &str) -> Result<()> {
    let (_, numbers) = all_consuming(parse_input)(file_data).unwrap();

    let number = numbers.iter().sum::<i64>();

    // for a in 0..=10 {
    //     println!("{:10} {:10}", a, number_to_snafu(a));
    // }
    // for a in [15, 20, 2022, 12345, 314159265] {
    //     println!("{:10} {:10}", a, number_to_snafu(a));
    // }

    dbg!(number_to_snafu(number));

    Ok(())
}

fn part2(_file_data: &str) -> Result<()> {
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
