//! Day 14
//!
//!

use adventofcode2022::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64 as nom_u64},
    multi::separated_list1,
    sequence::separated_pair,
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

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(u64, u64)>>> {
    separated_list1(
        line_ending,
        separated_list1(tag(" -> "), separated_pair(nom_u64, tag(","), nom_u64)),
    )(input)
}

enum GridContent {
    Wall,
    Sand,
}

struct Grid {
    grid: HashMap<(u64, u64), GridContent>,
    lowest_point: u64,
    floor: Option<u64>,
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self> {
        let (_, walls) = parse_input(input).unwrap();
        let mut grid = HashMap::new();
        let mut lowest_point = 0;
        for wall in walls {
            for start_dest in wall.windows(2) {
                if let [(sc, sr), (dc, dr)] = *start_dest {
                    for r in sr.min(dr)..=dr.max(sr) {
                        for c in (sc.min(dc))..=(dc.max(sc)) {
                            grid.insert((r, c), GridContent::Wall);
                        }
                        lowest_point = lowest_point.max(r);
                    }
                } else {
                    bail!("Couldn't break into windows");
                }
            }
        }
        Ok(Grid {
            grid,
            lowest_point,
            floor: None,
        })
    }
}

impl Grid {
    fn drop_sand(&mut self, mut sand_pos: (u64, u64)) -> Option<(u64, u64)> {
        loop {
            let dl = (sand_pos.0 + 1, sand_pos.1 - 1);
            let d = (sand_pos.0 + 1, sand_pos.1);
            let dr = (sand_pos.0 + 1, sand_pos.1 + 1);
            match (self.grid.get(&d), self.grid.get(&dl), self.grid.get(&dr)) {
                (None, _, _) => sand_pos = d,
                (_, None, _) => sand_pos = dl,
                (_, _, None) => sand_pos = dr,
                (Some(_), Some(_), Some(_)) => {
                    break;
                }
            }
            match self.floor {
                None => {
                    if sand_pos.0 > self.lowest_point {
                        return None;
                    }
                }
                Some(floor) => {
                    if sand_pos.0 == floor - 1 {
                        break;
                    }
                }
            }
        }
        self.grid.insert(sand_pos, GridContent::Sand);
        Some(sand_pos)
    }

    fn with_floor(self, height: u64) -> Self {
        let Grid {
            grid,
            lowest_point,
            floor: _,
        } = self;
        Self {
            grid,
            lowest_point,
            floor: Some(height),
        }
    }
}

fn part1(file_data: &str) -> Result<()> {
    let mut grid = Grid::try_from(file_data)?;
    let mut sand_units = 0;

    while grid.drop_sand((0, 500)).is_some() {
        sand_units += 1;
    }
    dbg!(sand_units);
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let grid = Grid::try_from(file_data)?;
    let floor = grid.lowest_point + 2;
    let mut grid = grid.with_floor(floor);
    let mut sand_units = 0;
    while let Some(pos) = grid.drop_sand((0, 500)) {
        sand_units += 1;
        if pos == (0, 500) {
            break;
        }
    }
    dbg!(sand_units);
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
