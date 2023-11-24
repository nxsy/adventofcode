// cargo run --example day17 -- (part1|part2) (example_input|final_input)

use std::fs::read_to_string;
use std::collections::HashSet;
use std::ops::RangeInclusive;

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
#[structopt(name = "day17", about = "Advent of Code 2021 Day 17")]
pub struct Args {
    part: Part,
    input: Input,
}

fn determine_hit(vel: (i32, i32), target: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> Option<i32> {
    let mut vel = vel;
    let mut pos = (0, 0);
    let mut max_y = 0;
    while &pos.1 > target.1.start() {
        pos.0 += vel.0;
        pos.1 += vel.1;
        max_y = max_y.max(pos.1);
        vel.0 -= vel.0.signum();
        vel.1 -= 1;

        if target.0.contains(&pos.0) && target.1.contains(&pos.1) {
            return Some(max_y);
        }
    }
    None
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;
    // target area: x=257..286, y=-101..-57
    let (_, contents) = contents.split_once(": ").unwrap();
    let (x, y) = contents.split_once(", ").unwrap();
    let (_, x) = x.split_once("=").unwrap();
    let (xstart, xstop) = x.split_once("..").unwrap();
    let (_, y) = y.split_once("=").unwrap();
    let (ystart, ystop) = y.split_once("..").unwrap();
    let xstart = xstart.parse::<i32>().unwrap();
    let xstop = xstop.parse::<i32>().unwrap();
    let ystart = ystart.parse::<i32>().unwrap();
    let ystop = ystop.parse::<i32>().unwrap();

    let target = (xstart..=xstop, ystart..=ystop);

    let mut min_steps = 0;
    while (min_steps * (min_steps + 1)) / 2 < *target.0.start() {
        min_steps += 1;
    }

    // Start roughly in the right place (min x, just below neutral y)
    let x = min_steps;
    let mut y = *target.1.start() / min_steps;

    let mut total_max_y = i32::MIN;
    loop {
        // Find neutral y
        if let Some(max_y) = determine_hit((x, y), &target) {
            total_max_y = total_max_y.max(max_y);
            break;
        }

        y += 1;
    }

    let mut stack = Vec::from([(x, y)]);
    let mut tried = HashSet::new();
    let mut hits = HashSet::new();
    while let Some(vel) = stack.pop() {
        if tried.contains(&vel) {
            continue;
        }
        tried.insert(vel);
        
        if  let Some(max_y) = determine_hit(vel, &target) {
            // println!("Hit at {:?} with max_y {}", vel, max_y);
            total_max_y = total_max_y.max(max_y);    
        } else {
            continue;    
        }
        hits.insert(vel);
        for dy in -10..10 {
            for dx in 0..400 {
                stack.push((vel.0 + dx, vel.1+dy));    
            }
        }
    }

    println!("max_y of {}, {} hits", total_max_y, hits.len());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day17/example_input",
        Input::FinalInput => "data/day17/input",
    };
    
    solve(file_path)
}