// cargo run --example day14 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
use std::fs::read_to_string;

use anyhow::Result;
use structopt::StructOpt;

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
#[structopt(name = "day14", about = "Advent of Code 2021 Day 9")]
pub struct Args {
    part: Part,
    input: Input,
}

fn str_to_pair_counts(s: &str) -> HashMap<String, i64> {
    let mut prev: Option<char> = None;
    let mut pair_numbers = HashMap::new();

    for c in s.chars() {
        match prev {
            None => {},
            Some(prev) => {
                pair_numbers
                    .entry(prev.to_string() + &c.to_string())
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                
            }
        }
        prev = Some(c);
    }

    pair_numbers
}

fn result_from_pair_counts(h: &HashMap<String, i64>, last_char: char) -> i64 {
    
    let mut element_count = HashMap::new();
    for (pair, number) in h.clone() {
        let c = pair.chars().next().unwrap();
            element_count
                .entry(c)
                .and_modify(|x| *x += number)
                .or_insert(number);
    }
    element_count.entry(last_char).and_modify(|x| *x += 1).or_insert(1);

    let mut elements_with_counts: Vec<_> = element_count.iter().map(|x| (*x.0, *x.1)).collect();
    elements_with_counts.sort_unstable_by_key(|x| x.1);
    let biggest = elements_with_counts.iter().last().unwrap();
    let smallest = elements_with_counts.get(0).unwrap();

    biggest.1 - smallest.1
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut past_break = false;
    let start = contents.lines().next().unwrap();
    let mut pair_insertion = HashMap::new();
    for line in contents.lines() {
        if line.trim() == "" {
            past_break = true;
            continue;
        }

        if past_break {
            let (pair, element) = line.split_once("->").unwrap();
            pair_insertion.insert(pair.trim().to_string(), element.trim().to_string());
        }
    }
    let last_char = start.chars().last().unwrap();

    let mut pair_numbers = str_to_pair_counts(start);
    let mut part1_numbers = HashMap::new();

    for step in 1..=40 {
        let mut new_pair_numbers = HashMap::new();
        for (pair, number) in pair_numbers {
            let inserted_char = pair_insertion.get(&pair).unwrap();

            {
                let char1 = pair.chars().next().unwrap();
                let s1 = char1.to_string() + inserted_char;
                new_pair_numbers
                    .entry(s1)
                    .and_modify(|x| *x += number)
                    .or_insert(number);
                
            }
            {
                let char2 = pair.chars().nth(1).unwrap();
                let s2 = inserted_char.clone() + &char2.to_string();
                new_pair_numbers
                    .entry(s2)
                    .and_modify(|x| *x += number)
                    .or_insert(number);
                
            }
        }
        pair_numbers = new_pair_numbers;
        if step == 10 {
            part1_numbers = pair_numbers.clone();
        }
    }

    println!("Part 1: {}", result_from_pair_counts(&part1_numbers, last_char));
    println!("Part 2: {}", result_from_pair_counts(&pair_numbers, last_char));

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day14/example_input",
        Input::FinalInput => "data/day14/input",
    };

    solve(file_path)
}
