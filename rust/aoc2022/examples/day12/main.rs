//! Day 12
//!
//!

use adventofcode2022::prelude::*;

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
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

struct Heightmap {
    width: usize,
    height: usize,
    elevations: HashMap<(i32, i32), u32>,
    start_pos: (i32, i32),
    end_pos: (i32, i32),
}

fn part1(file_data: &str) -> Result<()> {
    let Heightmap { width, height, elevations, start_pos, end_pos }  = build_heightmap(file_data);

    let mut costs = HashMap::new();
    for r in 0..height as i32 {
        for c in 0..width as i32 {
            costs.insert((r, c), usize::MAX);
        }
    }
    let mut stack = BinaryHeap::new();
    stack.push((0, start_pos));
    while let Some((cost, pos)) = stack.pop() {
        if cost >= costs[&pos] {
            continue;
        }
        costs.insert(pos, cost);
        let elevation = elevations[&pos];
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let n_pos = (pos.0 + dx, pos.1 + dy);
            if elevations.contains_key(&n_pos) {
                let new_elevation = elevations[&n_pos];
                if new_elevation > elevation + 1 {
                    continue;
                }
                stack.push((cost + 1, n_pos));
            }
        }
    }

    dbg!(costs[&end_pos]);
    Ok(())
}

fn build_heightmap(file_data: &str) -> Heightmap {
    let contents = file_data.to_string();
    let width = contents.lines().next().unwrap().chars().count();
    let height = contents.lines().count();
    let mut elevations = HashMap::new();
    let mut start_pos = None;
    let mut end_pos = None;
    for (r, line) in file_data.lines().enumerate() {
        let r = r as i32;
        for (c, char) in line.chars().enumerate() {
            let c = c as i32;
            let elevation_char = match char {
                'S' => {
                    start_pos = Some((r, c));
                    'a'
                }
                'E' => {
                    end_pos = Some((r, c));
                    'z'
                }
                char => char,
            };
            let elevation: u32 = elevation_char.into();
            elevations.insert((r, c), elevation);
        }
    }
    let start_pos = start_pos.unwrap();
    let end_pos = end_pos.unwrap();
    Heightmap { width, height, elevations, start_pos, end_pos }
}

fn part2(file_data: &str) -> Result<()> {
    let Heightmap { width, height, elevations, start_pos: _, end_pos }  = build_heightmap(file_data);

    let mut costs = HashMap::new();
    for r in 0..height as i32 {
        for c in 0..width as i32 {
            costs.insert((r, c), usize::MAX);
        }
    }
    let mut stack = BinaryHeap::new();
    stack.push((0, end_pos));
    while let Some((cost, pos)) = stack.pop() {
        if cost >= costs[&pos] {
            continue;
        }
        costs.insert(pos, cost);
        let elevation = elevations[&pos];
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let n_pos = (pos.0 + dx, pos.1 + dy);
            if elevations.contains_key(&n_pos) {
                let new_elevation = elevations[&n_pos];
                if new_elevation < elevation - 1 {
                    continue;
                }
                stack.push((cost + 1, n_pos));
            }
        }
    }

    let mut lowest = usize::MAX;
    for r in 0..height as i32 {
        for c in 0..width as i32 {
            if elevations[&(r, c)] == 'a'.into() {
                lowest = lowest.min(costs[&(r, c)]);
            }
        }
    }
    
    dbg!(lowest);
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
