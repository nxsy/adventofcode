//! Day 03
//!
//! Okay, finally caved and added itertools to the imports for "exactly one"
//! iterator handling.
//! 
//! Wasted a ton of time thinking `to_digit` would help (thinking it was
//! case-sensitive somehow.  Still find Rust's char -> u8 for ascii stuff
//! annoying.

use std::collections::HashSet;

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

#[derive(Debug)]
struct Rucksack {
    items1: Vec<Score>,
    items2: Vec<Score>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Score(u32);

impl From<char> for Score {
    fn from(c: char) -> Score {
        match c {
            'a'..='z' => Score(u32::from(c) - u32::from('a') + 1),
            'A'..='Z' => Score(u32::from(c) - u32::from('A') + 1 + 26),
            _ => unimplemented!(),
        }
    }
}

trait ToSet {
    fn to_set(&self) -> HashSet<Score>;
}
impl ToSet for Vec<Score> {
    fn to_set(&self) -> HashSet<Score> {
        HashSet::from_iter(self.iter().copied())
    }
}

fn part1(file_data: &str) -> Result<()> {
    let rucksacks = get_rucksacks(file_data);

    let mut total = 0;
    for rucksack in rucksacks {
        let s1 = rucksack.items1.to_set();
        let s2 = rucksack.items2.to_set();
        let score: u32 = s1.intersection(&s2).map(|x| x.0).sum();
        //println!("{s1:?} {s2:?} {score}");
        total += score;
    }
    println!("{total}");
    Ok(())
}

fn get_rucksacks(file_data: &str) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in file_data.lines() {
        let (items1, items2) = line.split_at(line.len() / 2);
        //println!("{items1} {items2}");
        let rucksack = Rucksack {
            items1: Vec::from_iter(items1.chars().map(Score::from)),
            items2: Vec::from_iter(items2.chars().map(Score::from)),
        };
        rucksacks.push(rucksack);
    }
    rucksacks
}


fn part2(file_data: &str) -> Result<()> {
    let rucksacks = get_rucksacks(file_data);

    let mut total = 0;
    for chunk in rucksacks.chunks(3) {
        let mut sets = Vec::new();
        for rucksack in chunk {
            let s1 = rucksack.items1.to_set();
            let s2 = rucksack.items2.to_set();    
            let union = &s1 | &s2;
            sets.push(union);
        }

        let mut remaining = sets[0].clone();
        for set in sets.iter().skip(1) {
            remaining = &remaining & set;
        }

        let score = remaining.into_iter().exactly_one()?;
        total += score.0;
    }
    println!("{total}");
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
