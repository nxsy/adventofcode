//! Day 18

use adventofcode2022::prelude::*;

use nom::{
    character::complete::{i64 as nom_i64, line_ending},
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

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    all_consuming(separated_list1(line_ending, nom_i64))(input)
}

fn part1(file_data: &str) -> Result<()> {
    let (_, input) = parse_input(file_data).unwrap();

    let s = mix(input, 1, 1);
    dbg!(s);

    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let (_, input) = parse_input(file_data).unwrap();

    let s = mix(input, 811589153, 10);
    dbg!(s);

    Ok(())
}

fn mix(input: Vec<i64>, key: i64, rounds: usize) -> i64 {
    let input = input.into_iter().map(|x| x * key).collect::<Vec<_>>();
    let mut indices = (0..input.len()).collect::<Vec<_>>();
    let mixer = input.clone();
    for _ in 0..rounds {
        for (i, &n) in input.iter().enumerate() {
            let p = indices.iter().position(|&x| x == i).unwrap();
            indices.remove(p);
            let new_p = (p as i64 + n).rem_euclid(indices.len() as i64) as usize;
            indices.insert(new_p, i);
        }
    }
    let input_p0 = input.iter().position(|&i| i == 0).unwrap();
    let index_p0 = indices.iter().position(|&x| x == input_p0).unwrap();
    let indices = [1000, 2000, 3000].map(|x| indices[(x + index_p0).rem_euclid(indices.len())]);
    let mixed = indices.iter().map(|i| mixer[*i]).collect::<Vec<_>>();
    let s = mixed.iter().sum::<i64>();
    s
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
