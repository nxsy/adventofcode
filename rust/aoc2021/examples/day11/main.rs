// cargo run --example day11 -- (part1|part2) (example_input|final_input)

use std::collections::hash_map::Entry;
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
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day11", about = "Advent of Code 2021 Day 6")]
pub struct Args {
    part: Part,
    input: Input,
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let rows = contents.lines().count() as i32;
    let columns = contents.lines().next().unwrap().chars().count() as i32;

    let mut grid = HashMap::new();

    let mut total_flashes = 0;
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }

    let mut explosions = Vec::new();
    let mut n = 0;
    loop {
        n += 1;
        let mut flashes = 0;

        for y in 0..rows {
            for x in 0..columns {
                grid.entry((x, y)).and_modify(|x| *x += 1);
                if grid[&(x, y)] > 9 {
                    explosions.push((x, y));
                }
            }
        }

        let mut exploded = HashSet::new();
        while let Some(ep) = explosions.pop() {
            let (x, y) = ep;
            if exploded.contains(&ep) {
                continue;
            }
            flashes += 1;
            exploded.insert(ep);
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let pos = &(x + dx, y + dy);
                    if exploded.contains(pos) {
                        continue;
                    }

                    if let Entry::Occupied(mut entry) = grid.entry(*pos) {
                        let v = entry.get_mut();
                        *v += 1;
                        if *v > 9 {
                            explosions.push(*pos);
                            *v = 0;
                        }
                    }
                }
            }
        }

        for e in exploded {
            grid.entry(e).and_modify(|x| *x = 0);
        }

        if n <= 100 {
            total_flashes += flashes;
        }

        if flashes == rows * columns {
            println!("Full flash at {}", n);
            break;
        }
    }

    println!("final result: {}", total_flashes);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day11/example_input",
        Input::FinalInput => "data/day11/input",
    };
    solve(file_path)
}
