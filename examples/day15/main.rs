// cargo run --example day15 -- (part1|part2) (example_input|final_input)

use std::collections::BinaryHeap;
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
#[structopt(name = "day15", about = "Advent of Code 2021 Day 9")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(PartialEq, Eq)]
struct Cost(usize);

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|o| o.reverse())
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

fn solve(file_path: &str, part: Part) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let input_width = contents.lines().next().unwrap().len() as i32;
    let input_height = contents.lines().count() as i32;
    let multiple = match part {
        Part::Part1 => 1,
        Part::Part2 => 5,
    };
    let width = input_width * multiple;
    let height = input_height * multiple;

    let mut g = HashMap::new();
    let mut costs = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let orig_cost = c.to_digit(10).unwrap() as i32;
            for rx in 0..multiple {
                for ry in 0..multiple {
                    let mut rcost = (orig_cost + rx + ry) % 9;
                    if rcost == 0 {
                        rcost = 9;
                    }
                    let pos = (x as i32 + rx * input_width, y as i32 + ry * input_height);
                    g.insert(pos, rcost as usize);
                    costs.insert(pos, usize::MAX);
                }
            }
        }
    }

    let start = (0, 0);

    let mut stack = BinaryHeap::new();
    stack.push((Cost(0), start));
    while let Some((Cost(cost), pos)) = stack.pop() {
        if cost >= costs[&pos] {
            continue;
        }
        costs.insert(pos, cost);
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let n_pos = (pos.0 + dx, pos.1 + dy);
            if g.contains_key(&n_pos) {
                stack.push((Cost(cost + g[&n_pos]), n_pos));
            }
        }
    }

    let p = (height - 1, width - 1);
    println!("{}", costs[&p]);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day15/example_input",
        Input::FinalInput => "data/day15/input",
    };

    solve(file_path, args.part)
}
