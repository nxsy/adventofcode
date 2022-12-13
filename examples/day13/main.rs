//! Day 13
//!
//!

use std::cmp::Ordering;

use adventofcode2022::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64 as nom_u64},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListItem {
    Number(u64),
    List(Vec<ListItem>),
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListItem::Number(a), ListItem::Number(b)) => a.cmp(b),
            (ListItem::List(a), ListItem::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    let cmp = match (a.next(), b.next()) {
                        (None, None) => break,
                        (None, Some(_)) => Ordering::Less,
                        (Some(_), None) => Ordering::Greater,
                        (Some(a), Some(b)) => a.cmp(b),
                    };
                    match cmp {
                        Ordering::Equal => {}
                        c => return c,
                    }
                }
                Ordering::Equal
            }
            (ListItem::Number(a), _) => ListItem::List(vec![ListItem::Number(*a)]).cmp(other),
            (_, ListItem::Number(b)) => self.cmp(&ListItem::List(vec![ListItem::Number(*b)])),
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list_item(input: &str) -> IResult<&str, ListItem> {
    if let Ok((input, l)) = delimited(
        tag("["),
        separated_list0(tag(","), parse_list_item),
        tag("]"),
    )(input)
    {
        Ok((input, ListItem::List(l)))
    } else {
        let (input, n) = nom_u64(input)?;
        Ok((input, ListItem::Number(n)))
    }
}

fn part1(file_data: &str) -> Result<()> {
    let (_, pairs) = separated_list1(
        pair(line_ending, line_ending),
        separated_pair(parse_list_item, line_ending, parse_list_item),
    )(file_data)
    .unwrap();
    let mut correct = HashSet::new();
    for (i, (left, right)) in pairs.iter().enumerate() {
        match left.cmp(right) {
            Ordering::Equal => unimplemented!(),
            Ordering::Less => {
                correct.insert(i);
            }
            Ordering::Greater => {}
        }
    }
    println!("{}", correct.iter().map(|x| x + 1).sum::<usize>());
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let (_, mut pairs) = separated_list1(many1(line_ending), parse_list_item)(file_data).unwrap();
    let (_, separator1) = parse_list_item("[[2]]").unwrap();
    let (_, separator2) = parse_list_item("[[6]]").unwrap();
    pairs.push(separator1.clone());
    pairs.push(separator2.clone());
    pairs.sort();
    let decoder_key = (pairs.iter().position(|x| x == &separator1).unwrap() + 1)
        * (pairs.iter().position(|x| x == &separator2).unwrap() + 1);
    dbg!(decoder_key);
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
