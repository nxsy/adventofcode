// cargo run --example day9 -- (part1|part2) (example_input|final_input)

use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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
#[structopt(name = "day9", about = "Advent of Code 2021 Day 9")]
pub struct Args {
    part: Part,
    input: Input,
}

fn part1(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let width = contents.lines().next().unwrap().chars().count();
    let height = contents.lines().count();

    let mut flat_map: Vec<usize> = Vec::new();
    for line in contents.lines() {
        let mut line_vec: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
        flat_map.append(&mut line_vec);
    }

    let mut risk = 0;
    for (n, i) in flat_map.iter().enumerate() {
        let options = &[
            if n < width { None } else { Some(n - width) },
            if (n % width) == 0 { None } else { Some(n - 1) },
            if n > width * (height - 1) - 1 { None } else { Some(n + width) },
            if (n % width) == width - 1 { None } else { Some (n + 1)},
        ];
        let mut low_point = true;
        for d in options.iter().flatten().copied() {
            if *i >= flat_map[d] {
                low_point = false;
                break;
            }
        }
        if low_point {
            risk += i + 1;
        }

    }
    println!("Risk: {}", risk);
    Ok(())
}

fn part2(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let width = contents.lines().next().unwrap().chars().count();
    let height = contents.lines().count();

    let mut flat_map: Vec<usize> = Vec::new();
    for line in contents.lines() {
        let mut line_vec: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
        flat_map.append(&mut line_vec);
    }

    let mut basins: Vec<HashSet<usize>> = Vec::new();
    let mut assigned: HashSet<usize> = HashSet::new();
    let mut basin_sizes = BinaryHeap::new();
    for (n, i) in flat_map.iter().copied().enumerate() {
        if assigned.contains(&n) {
            continue;
        }
        if i == 9 {
            // not actually assigned, but ignore it in future
            assigned.insert(n);
            continue;
        }
        let mut basin = HashSet::new();
        let mut unexplored: HashSet<usize> = HashSet::new();
        unexplored.insert(n);
        while let Some(n) = unexplored.iter().next().copied() {
            // println!("{}", n);
            basin.insert(n);
            assigned.insert(n);
            unexplored.remove(&n);

            let options = &[
                if n < width { None } else { Some(n - width) },
                if (n % width) == 0 { None } else { Some(n - 1) },
                if n > width * (height - 1) - 1 { None } else { Some(n + width) },
                if (n % width) == width - 1 { None } else { Some (n + 1)},
            ];
            for option in options.iter().flatten().copied() {
                if assigned.contains(&option) {
                    continue;
                }
                if flat_map[option] == 9 {
                    assigned.insert(option);
                    continue;
                }
                unexplored.insert(option);
            }
        }
        basin_sizes.push(basin.len());
        basins.push(basin);
    }
    println!("Top basin sizes: {}", basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day9/example_input",
        Input::FinalInput => "data/day9/input",
    };
    
    match args.part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}