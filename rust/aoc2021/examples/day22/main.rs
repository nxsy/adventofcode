// cargo run --example day22 -- (part1|part2) (example_input|final_input)

use std::collections::HashSet;
use std::fs::read_to_string;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
    ExampleInput2,
    ExampleInput3,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day22", about = "Advent of Code 2021 Day 22")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug, Hash, Eq, Clone, Copy, PartialEq, PartialOrd, Ord)]
struct Cube {
    // exclusive
    xrange: (i64, i64),
    yrange: (i64, i64),
    zrange: (i64, i64),
}

impl Cube {
    fn overlap(&self, other: &Cube) -> bool {
        self.xrange.0 < other.xrange.1
            && self.yrange.0 < other.yrange.1
            && self.zrange.0 < other.zrange.1
            && self.xrange.1 > other.xrange.0
            && self.yrange.1 > other.yrange.0
            && self.zrange.1 > other.zrange.0
    }

    fn contained_in(&self, other: &Cube) -> bool {
        self.xrange.0 >= other.xrange.0
            && self.xrange.1 <= other.xrange.1
            && self.yrange.0 >= other.yrange.0
            && self.yrange.1 <= other.yrange.1
            && self.zrange.0 >= other.zrange.0
            && self.zrange.1 <= other.zrange.1
    }

    fn volume(&self) -> i64 {
        (self.xrange.1 - self.xrange.0)
            * (self.yrange.1 - self.yrange.0)
            * (self.zrange.1 - self.zrange.0)
    }
}

fn count_ons(world: Cube, cubes: &[(Cube, bool)]) -> i64 {
    let mut final_overlap = None;
    for &(other, on) in cubes.iter() {
        if world.overlap(&other) {
            final_overlap = Some((other, on));
        }
    }

    if final_overlap.is_none() {
        return 0;
    }

    let (final_overlap, on) = final_overlap.unwrap();

    if world.contained_in(&final_overlap) {
        if !on {
            return 0;
        } else {
            return world.volume();
        }
    }

    if world.volume() == 1 {
        return 0;
    }

    let xs = overlaps(world.xrange, final_overlap.xrange);
    let ys = overlaps(world.yrange, final_overlap.yrange);
    let zs = overlaps(world.zrange, final_overlap.zrange);

    let mut ons = 0;
    for &xrange in &xs {
        for &yrange in &ys {
            for &zrange in &zs {
                let new_world = Cube {
                    xrange,
                    yrange,
                    zrange,
                };
                ons += count_ons(new_world, cubes);
            }
        }
    }

    ons
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;
    let lines = contents.lines();

    let mut cubes = Vec::new();

    let mut xrange = (0, 0);
    let mut yrange = (0, 0);
    let mut zrange = (0, 0);
    for line in lines {
        // on x=10..12,y=10..12,z=10..12
        let (instr, coords) = line.split_once(' ').unwrap();
        let coords: Vec<_> = coords
            .split(',')
            .map(|f| f.split_once('=').unwrap().1)
            .map(|f| {
                let (s, e) = f.split_once("..").unwrap();
                (
                    s.parse::<i64>().unwrap(),
                    e.parse::<i64>().unwrap() + 1, // range exclusive
                )
            })
            .collect();
        xrange.0 = xrange.0.min(coords[0].0);
        xrange.1 = xrange.1.max(coords[0].1);
        yrange.0 = yrange.0.min(coords[1].0);
        yrange.1 = yrange.1.max(coords[1].1);
        zrange.0 = zrange.0.min(coords[2].0);
        zrange.1 = zrange.1.max(coords[2].1);

        let c = Cube {
            xrange: (coords[0].0, coords[0].1),
            yrange: (coords[1].0, coords[1].1),
            zrange: (coords[2].0, coords[2].1),
        };
        cubes.push((c, instr == "on"));
    }

    let part1_world = Cube {
        xrange: (-50, 51),
        yrange: (-50, 51),
        zrange: (-50, 51),
    };
    let part2_world = Cube {
        xrange,
        yrange,
        zrange,
    };

    let part1_ons = count_ons(part1_world, &cubes);
    let part2_ons = count_ons(part2_world, &cubes);

    println!("Part 1: {}", part1_ons);
    println!("Part 2: {}", part2_ons);

    Ok(())
}

fn overlaps(outer: (i64, i64), other: (i64, i64)) -> Vec<(i64, i64)> {
    let mut v: Vec<_> = HashSet::from([outer.0, other.0, other.1, outer.1])
        .into_iter()
        .filter(|p| *p <= outer.1 && *p >= outer.0)
        .collect();
    v.sort_unstable();
    let mut res = Vec::new();
    for s in v.windows(2) {
        res.push((s[0], s[1]));
    }
    res
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day22/example_input",
        Input::FinalInput => "data/day22/input",
        Input::ExampleInput2 => "data/day22/example_input2",
        Input::ExampleInput3 => "data/day22/example_input3",
    };

    solve(file_path)
}
