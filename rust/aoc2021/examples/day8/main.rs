// cargo run --example day8 -- (part1|part2) (example_input|final_input)

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

segments | numbers
       2 | 1
       3 | 7
       4 | 4
       5 | 2, 3, 5
       6 | 0, 6, 9
       7 | 8

- 1, 4, 7, and 8 are unique by number of segments.
(1, 4, 7, 8)
Segment f is unique, since it is present in all numbers but 2 (which makes 2 unique).
(1, 2, 4, 7, 8)
9 is the only 6-segment number which contains all the segments 4 has.
(1, 2, 4, 7, 8, 9)
5: segments from 2 + 5 make 8
(1, 2, 4, 5, 7, 8, 9)
3: remaining 5 segment number
(1, 2, 3, 4, 5, 7, 8, 9)
6: 6 union 5 = 6 (and check that not 9)
0: remaining number

part 1: 369
part 2: 1031553

 */

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString, PartialEq, Eq, PartialOrd, Ord)]
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
#[structopt(name = "day8", about = "Advent of Code 2021 Day 6")]
pub struct Args {
    part: Part,
    input: Input,
}

fn extract_signals(s: &str) -> Vec<HashSet<char>> {
    return s.trim()
            .split_whitespace()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect();
}

fn part1(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut final_result = 0;
    for line in contents.lines() {
        let (left, right) = line.split_once("|").unwrap();

        let signal_patterns = extract_signals(left);
        let mut numbers_to_signal: HashMap<i32, &HashSet<char>> = HashMap::new();
        let mut segment_counts: HashMap<char, i32> = HashMap::new();
        
        for signal in signal_patterns.iter() {
            let segment_match = match signal.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            };
            if let Some(segment_match) = segment_match {
                if numbers_to_signal.contains_key(&segment_match) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(segment_match, signal);
            }
            for segment in signal {
                segment_counts.entry(*segment).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        let mut segment_counts_vec: Vec<_> = segment_counts.iter().map(|(k, v)| (*k, *v)).collect();
        segment_counts_vec.sort_unstable_by_key(|(_, v)| 8 - *v);
        let most_common_segment = segment_counts_vec[0].0;
        for signal in signal_patterns.iter() {
            if !signal.contains(&most_common_segment) {
                if numbers_to_signal.contains_key(&2) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(2, signal);
            }

            if signal.len() == 6 && numbers_to_signal[&4].difference(signal).count() == 0 {
                if numbers_to_signal.contains_key(&9) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(9, signal);
            }
        }

        for signal in signal_patterns.iter() {
            if signal.len() == 5 && numbers_to_signal[&2].union(signal).count() == 7 {
                if numbers_to_signal.contains_key(&5) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(5, signal);
            }
        }

        for signal in signal_patterns.iter() {
            if signal.len() == 5 && signal != numbers_to_signal[&5] && signal != numbers_to_signal[&2] {
                if numbers_to_signal.contains_key(&3) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(3, signal);
            }

            if signal.len() == 6 && HashSet::from_iter(numbers_to_signal[&5].union(signal).copied()).eq(signal) && signal != numbers_to_signal[&9] {
                if numbers_to_signal.contains_key(&6) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(6, signal);
            }
        }

        for signal in signal_patterns.iter() {
            if signal.len() == 6 && signal != numbers_to_signal[&6] && signal != numbers_to_signal[&9] {
                if numbers_to_signal.contains_key(&0) {
                    panic!("This was already added!");
                }
                numbers_to_signal.insert(0, signal);
            }
        }

        // println!("Numbers to signals: {:#?}", numbers_to_signal);

        let out_signal_patterns = extract_signals(right);

        let mut result = 0;
        'a: for (i, n) in out_signal_patterns.iter().enumerate() {
            for (k, v) in numbers_to_signal.iter() {
                if n == *v {
                    result += *k as i32 * 10_i32.pow(3 - i as u32);
                    continue 'a;
                }
            }
            panic!("Reached end of block without finding signal?");
        }

        println!("Result: {}", result);
        final_result += result;
    }

    println!("final result: {}", final_result);

    Ok(())
}

fn main() -> Result<()> {

    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day8/example_input",
        Input::FinalInput => "data/day8/input",
    };
    part1(file_path)
}
