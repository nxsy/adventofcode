// cargo run --example day20 -- (part1|part2) (example_input|final_input)

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
#[structopt(name = "day20", about = "Advent of Code 2021 Day 20")]
pub struct Args {
    part: Part,
    input: Input,
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut lines = contents.lines();

    let mut algorithm = String::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let line = line
            .chars()
            .map(|c| if c == '.' { '0' } else { '1' })
            .collect::<String>();
        algorithm.push_str(&line);
    }

    let algorithm = algorithm.chars().collect::<Vec<_>>();

    let mut grid_bounds = ((0, 0), (0, 0));
    let mut grid = HashMap::new();
    for (i, line) in (0..).zip(lines) {
        grid_bounds.1 .0 = grid_bounds.1 .0.min(i);
        grid_bounds.1 .1 = grid_bounds.1 .1.max(i);
        for (j, c) in (0..).zip(line.chars()) {
            grid_bounds.0 .0 = grid_bounds.0 .0.min(j);
            grid_bounds.0 .1 = grid_bounds.0 .1.max(j);
            grid.insert((j, i), if c == '.' { '0' } else { '1' });
        }
    }

    let mut border = '0';
    let surrounding = (-1..=1)
        .flat_map(move |y| (-1..=1).map(move |x| -> (i32, i32) { (x, y) }))
        .collect::<Vec<_>>();

    let mut iter_2_grid = grid.clone();
    for iter in 0..50 {
        let mut new_grid = HashMap::new();
        grid_bounds = (
            (grid_bounds.0 .0 - 1, grid_bounds.0 .1 + 1),
            (grid_bounds.1 .0 - 1, grid_bounds.1 .1 + 1),
        );
        for y in grid_bounds.1 .0..=grid_bounds.1 .1 {
            for x in grid_bounds.0 .0..=grid_bounds.0 .1 {
                let chars = surrounding
                    .iter()
                    .map(|(dx, dy)| grid.get(&(x + dx, y + dy)).unwrap_or(&border))
                    .collect::<String>();
                let idx = usize::from_str_radix(&chars, 2).unwrap();
                new_grid.insert((x, y), algorithm[idx]);
            }
        }
        border = algorithm[usize::from_str_radix(&border.to_string().repeat(9), 2).unwrap()];

        grid = new_grid;
        if iter == 1 {
            iter_2_grid = grid.clone();
        }
    }

    println!(
        "Part A: {}",
        iter_2_grid.iter().filter(|(_, &c)| c == '1').count()
    );
    println!("Part B: {}", grid.iter().filter(|(_, &c)| c == '1').count());

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day20/example_input",
        Input::FinalInput => "data/day20/input",
    };

    solve(file_path)
}
