// cargo run --example day12 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
use std::collections::HashSet;
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
#[allow(clippy::enum_variant_names)]
enum Input {
    ExampleInput,
    FinalInput,
    ExampleInput2,
    ExampleInput3,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day12", about = "Advent of Code 2021 Day 6")]
pub struct Args {
    part: Part,
    input: Input,
}

fn is_big(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

struct StackEntry<'a> {
    loc: &'a str,
    path: Vec<&'a str>,
    visited: HashSet<&'a str>,
    double_visit: bool,
}

fn solve(file_path: &str, part: Part) -> Result<()> {
    let contents = read_to_string(file_path).unwrap();

    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let (left, right) = line.split_once('-').unwrap();
        adjacency.entry(left).or_default().push(right);
        adjacency.entry(right).or_default().push(left);
    }

    let mut stack = Vec::from([StackEntry {
        loc: "start",
        path: Vec::from(["start"]),
        visited: HashSet::from(["start"]),
        double_visit: match part {
            Part::Part1 => false,
            Part::Part2 => true,
        }
    }]);

    let mut paths: Vec<Vec<&str>> = Vec::new();
    while let Some(entry) = stack.pop() {
        for &option in &adjacency[entry.loc] {
            if option == "start" {
                continue;
            }
            if option == "end" {
                paths.push(entry.path.clone());
                continue;
            }
            let mut visited = entry.visited.clone();
            let mut double_visit = entry.double_visit;
            if !is_big(option) {
                if visited.contains(option) {
                    if double_visit {
                        double_visit = false;
                    } else {
                        continue;
                    }
                }
                visited.insert(option);
            }
            let mut path = entry.path.clone();
            path.push(option);
            stack.push(StackEntry {
                loc: option,
                path,
                visited,
                double_visit,
            })
        }
    }

    let num_paths = paths.len();
    println!("Number of paths: {}", num_paths);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day12/example_input",
        Input::ExampleInput2 => "data/day12/example_input2",
        Input::ExampleInput3 => "data/day12/example_input3",
        Input::FinalInput => "data/day12/input",
    };

    solve(file_path, args.part)
}
