// cargo run --example day10 -- (part1|part2) (example_input|final_input)

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
#[structopt(name = "day10", about = "Advent of Code 2021 Day 9")]
pub struct Args {
    part: Part,
    input: Input,
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;
    let mut ascore = 0;
    let mut bscores = Vec::new();
    let matching_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let score_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let completion_score_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    for line in contents.lines() {
        let mut line_score = 0;

        let mut stack = Vec::new();
        'a: for char in line.trim().chars() {
            if matching_map.contains_key(&char) {
                stack.push(char);
            } else {
                for (&k, &v) in &matching_map {
                    if v == char {
                        if let Some(&x) = stack.last() {
                            if x == k {
                                stack.pop();
                                break;
                            }
                            line_score += score_map[&char];
                            break 'a;
                        } else {
                            // ...
                        }
                    }
                }
            }
        }

        if line_score == 0 {
            let mut line_score2: i64 = 0;
            while let Some(c) = stack.pop() {
                line_score2 *= 5;
                line_score2 += completion_score_map[&c];
            }
            bscores.push(line_score2);
        }
        ascore += line_score;
    }

    println!("Part1: {}", ascore);
    bscores.sort_unstable();
    println!("Part 2: {}", bscores[bscores.len() / 2]);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day10/example_input",
        Input::FinalInput => "data/day10/input",
    };

    solve(file_path)
}
