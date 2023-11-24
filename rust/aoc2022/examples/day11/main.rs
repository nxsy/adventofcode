//! Day 11
//!
//!

use adventofcode2022::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, u64 as nom_u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, tuple},
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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible: u64,
    true_monkey: u64,
    false_monkey: u64,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("new = ")(input)?;
    let (input, (b, c)) = alt((
        preceded(tag("old"), tuple((tag(" * "), tag("old")))),
        preceded(tag("old"), tuple((tag(" * "), digit1))),
        preceded(tag("old"), tuple((tag(" + "), digit1))),
    ))(input)?;
    let operation = match (b, c) {
        (" * ", "old") => Operation::Square,
        (" * ", n) => Operation::Mul(nom_u64(n)?.1),
        (" + ", n) => Operation::Add(nom_u64(n)?.1),
        _ => unreachable!(),
    };
    Ok((input, operation))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), nom_u64, tag(":")))(input)?;
    let (input, items) = preceded(
        tuple((line_ending, tag("  Starting items: "))),
        separated_list1(tag(", "), nom_u64),
    )(input)?;
    let (input, operation) =
        preceded(tuple((line_ending, tag("  Operation: "))), parse_operation)(input)?;
    let (input, divisible) =
        preceded(tuple((line_ending, tag("  Test: divisible by "))), nom_u64)(input)?;
    let (input, true_monkey) = preceded(
        tuple((line_ending, tag("    If true: throw to monkey "))),
        nom_u64,
    )(input)?;
    let (input, false_monkey) = preceded(
        tuple((line_ending, tag("    If false: throw to monkey "))),
        nom_u64,
    )(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let monkey = Monkey {
        items,
        operation,
        divisible,
        true_monkey,
        false_monkey,
    };
    Ok((input, monkey))
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    separated_list1(line_ending, parse_monkey)(input).unwrap().1
}

fn part1(file_data: &str) -> Result<()> {
    let mut monkeys = parse_monkeys(file_data);
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            let Monkey {
                true_monkey,
                false_monkey,
                operation,
                divisible,
                ..
            } = monkeys[monkey];
            let items = drain_items(&mut monkeys, monkey, operation, divisible, true);
            inspections[monkey] += items.len();
            for (d, item) in items {
                let add_to_monkey =
                    &mut monkeys[if d { true_monkey } else { false_monkey } as usize];
                add_to_monkey.items.push(item);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    dbg!(inspections[0] * inspections[1]);
    Ok(())
}

fn drain_items(
    monkeys: &mut [Monkey],
    monkey: usize,
    operation: Operation,
    divisible: u64,
    relief: bool,
) -> Vec<(bool, u64)> {
    let mut items = Vec::new();
    for mut item in monkeys[monkey].items.drain(..) {
        match operation {
            Operation::Add(n) => item += n,
            Operation::Mul(n) => item *= n,
            Operation::Square => item *= item,
        }
        if relief {
            item /= 3;
        }
        items.push((item % divisible == 0, item))
    }
    items
}

fn part2(file_data: &str) -> Result<()> {
    let mut monkeys = parse_monkeys(file_data);
    let mut inspections = vec![0; monkeys.len()];

    let divisor: u64 = monkeys.iter().map(|m| m.divisible).product();

    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            let Monkey {
                true_monkey,
                false_monkey,
                operation,
                divisible,
                ..
            } = monkeys[monkey];
            let items = drain_items(&mut monkeys, monkey, operation, divisible, false);
            inspections[monkey] += items.len();
            for (d, mut item) in items {
                item %= divisor;
                let add_to_monkey =
                    &mut monkeys[if d { true_monkey } else { false_monkey } as usize];
                add_to_monkey.items.push(item);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    dbg!(inspections[0] * inspections[1]);
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
