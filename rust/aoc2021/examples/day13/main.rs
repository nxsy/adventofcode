// cargo run --example day13 -- (part1|part2) (example_input|final_input)

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
#[structopt(name = "day13", about = "Advent of Code 2021 Day 6")]
pub struct Args {
    part: Part,
    input: Input,
}

enum Axis {
    X,
    Y,
}

fn do_fold(pos: &(i32, i32), fold: &(Axis, i32)) -> (i32, i32) {
    match fold.0 {
        Axis::X => {
            if pos.0 > fold.1 {
                (fold.1 - (pos.0 - fold.1), pos.1)
            } else {
                *pos
            }
        }
        Axis::Y => {
            if pos.1 > fold.1 {
                (pos.0, fold.1 - (pos.1 - fold.1))
            } else {
                *pos
            }
        }
    }
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path).unwrap();

    let mut h = HashSet::new();
    let mut in_fold = false;
    let mut folds = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            in_fold = true;
            continue;
        }
        if !in_fold {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            h.insert((x, y));
        } else {
            let instr = line.split(' ').nth(2).unwrap();
            let (axis, number) = instr.split_once('=').unwrap();
            let number = number.parse::<i32>().unwrap();
            let axis = match axis {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("No such axis"),
            };
            folds.push((axis, number));
        }
    }

    for (n, fold) in folds.iter().enumerate() {
        if n == 1 {
            println!("Part 1 answer is: {}", h.len());
        }
        for pos in h.clone() {
            let new_pos = do_fold(&pos, fold);
            if new_pos != pos {
                h.remove(&pos);
                h.insert(new_pos);
            }
        }
    }

    let (max_x, max_y) = h
        .iter()
        .copied()
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = match h.get(&(x, y)) {
                Some(_) => "#",
                None => ".",
            };
            print!("{}", c);
        }
        println!();
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day13/example_input",
        Input::FinalInput => "data/day13/input",
    };
    solve(file_path)
}
