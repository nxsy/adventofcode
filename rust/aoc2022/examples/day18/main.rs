//! Day 18

use adventofcode2022::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Pos(i64, i64, i64);

impl Pos {
    fn nearby_positions(&self) -> Vec<Pos> {
        let mut res = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if (x == 0) as u32 + (y == 0) as u32 + (z == 0) as u32 != 2 {
                        continue;
                    }
                    res.push(Pos(self.0 + x, self.1 + y, self.2 + z))
                }
            }
        }
        res
    }
}

fn parse_line(input: &str) -> IResult<&str, Pos> {
    let (input, (x, y, z)) = tuple((
        nom_i64,
        preceded(tag(","), nom_i64),
        preceded(tag(","), nom_i64),
    ))(input)?;
    Ok((input, Pos(x, y, z)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pos>> {
    separated_list1(line_ending, parse_line)(input)
}

fn part1(file_data: &str) -> Result<()> {
    let (_, pos) = parse_input(file_data).unwrap();

    let mut grid = HashSet::new();
    for p in &pos {
        grid.insert(*p);
    }

    let mut surface = 0;
    for p in pos {
        for sp in p.nearby_positions() {
            if !grid.contains(&sp) {
                surface += 1;
            }
        }
    }
    dbg!(surface);
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let (_, pos) = parse_input(file_data).unwrap();

    let mut grid = HashSet::new();
    let mut bounds = (
        Pos(i64::max_value(), i64::max_value(), i64::max_value()),
        Pos(i64::min_value(), i64::min_value(), i64::min_value()),
    );

    for p in &pos {
        grid.insert(*p);
        bounds.0 .0 = bounds.0 .0.min(p.0);
        bounds.1 .0 = bounds.1 .0.max(p.0);
        bounds.0 .1 = bounds.0 .1.min(p.1);
        bounds.1 .1 = bounds.1 .1.max(p.1);
        bounds.0 .2 = bounds.0 .2.min(p.2);
        bounds.1 .2 = bounds.1 .2.max(p.2);
    }
    dbg!(bounds);

    let mut potential_external = VecDeque::new();
    let mut known_external = HashSet::new();
    for x in bounds.0 .0 - 1..=bounds.1 .0 + 1 {
        for y in bounds.0 .1 - 1..=bounds.1 .1 + 1 {
            for z in bounds.0 .2 - 1..=bounds.1 .2 + 1 {
                if x == bounds.0 .0 - 1
                    || x == bounds.1 .0 + 1
                    || y == bounds.0 .1 - 1
                    || y == bounds.1 .1 + 1
                    || z == bounds.0 .2 - 1
                    || z == bounds.1 .2 + 1
                {
                    potential_external.push_back(Pos(x, y, z));
                }
            }
        }
    }

    while let Some(p) = potential_external.pop_front() {
        if grid.contains(&p) {
            continue;
        }
        if known_external.contains(&p) {
            continue;
        }
        known_external.insert(p);
        for sp in p.nearby_positions() {
            if sp.0 < bounds.0 .0
                || sp.0 > bounds.1 .0
                || sp.1 < bounds.0 .1
                || sp.1 > bounds.1 .1
                || sp.2 < bounds.0 .2
                || sp.2 > bounds.1 .2
            {
                continue;
            }
            if known_external.contains(&sp) {
                continue;
            }
            if grid.contains(&sp) {
                continue;
            }
            potential_external.push_back(sp);
        }
    }

    let mut surface = 0;
    for p in pos {
        for sp in p.nearby_positions() {
            if !grid.contains(&sp) && known_external.contains(&sp) {
                surface += 1;
            }
        }
    }
    dbg!(surface);
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
