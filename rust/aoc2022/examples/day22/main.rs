//! Day 22

use adventofcode2022::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    multi::many1,
    sequence::{separated_pair, terminated},
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Instruction {
    Forward(i64),
    TurnRight,
    TurnLeft,
}

fn parse_map_line(input: &str) -> IResult<&str, HashMap<i64, Tile>> {
    let (input, chars) = many1(alt((tag(" "), tag("."), tag("#"))))(input)?;
    let mut h = HashMap::new();
    for (i, c) in chars.into_iter().enumerate() {
        if let Some(tile) = match c {
            " " => None,
            "." => Some(Tile::Open),
            "#" => Some(Tile::Wall),
            _ => unreachable!(),
        } {
            h.insert(i as i64, tile);
        }
    }
    Ok((input, h))
}

fn parse_map(input: &str) -> IResult<&str, HashMap<(i64, i64), Tile>> {
    let (input, map_tiles) = many1(terminated(parse_map_line, line_ending))(input)?;
    let mut h = HashMap::new();
    for (i, l) in map_tiles.into_iter().enumerate() {
        for (k, v) in l {
            h.insert((i as i64, k), v);
        }
    }
    Ok((input, h))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = many1(alt((digit1, tag("L"), tag("R"))))(input)?;

    let mut r = vec![];

    for i in instructions {
        r.push(match i {
            "L" => Instruction::TurnLeft,
            "R" => Instruction::TurnRight,
            digits => Instruction::Forward(digits.parse::<i64>().unwrap()),
        })
    }
    Ok((input, r))
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> IResult<&str, (HashMap<(i64, i64), Tile>, Vec<Instruction>)> {
    separated_pair(parse_map, line_ending, parse_instructions)(input)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}
impl Dir {
    fn turned(&self, instr: Instruction) -> Dir {
        match (self, instr) {
            (_, Instruction::Forward(_)) => *self,
            (Dir::Up, Instruction::TurnRight) => Dir::Right,
            (Dir::Right, Instruction::TurnRight) => Dir::Down,
            (Dir::Down, Instruction::TurnRight) => Dir::Left,
            (Dir::Left, Instruction::TurnRight) => Dir::Up,
            (Dir::Up, Instruction::TurnLeft) => Dir::Left,
            (Dir::Left, Instruction::TurnLeft) => Dir::Down,
            (Dir::Down, Instruction::TurnLeft) => Dir::Right,
            (Dir::Right, Instruction::TurnLeft) => Dir::Up,
        }
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
        }
    }

    fn score(&self) -> i64 {
        match self {
            Dir::Up => 3,
            Dir::Left => 2,
            Dir::Down => 1,
            Dir::Right => 0,
        }
    }
}

fn part1(file_data: &str) -> Result<()> {
    let (_, (grid, instructions)) = parse_input(file_data).unwrap();

    let mut pos = *grid.keys().sorted_by_key(|x| (x.0, x.1)).next().unwrap();
    let mut dir = Dir::Right;

    for instr in instructions {
        match instr {
            Instruction::Forward(x) => {
                let delta = dir.delta();
                for _ in 0..x {
                    let test_pos = (pos.0 + delta.0, pos.1 + delta.1);
                    if let Some(Tile::Wall) = grid.get(&test_pos) {
                        break;
                    }
                    if let Some(Tile::Open) = grid.get(&test_pos) {
                        pos = test_pos;
                        continue;
                    }
                    let new_pos = match dir {
                        Dir::Up => *grid
                            .keys()
                            .filter(|x| x.1 == pos.1)
                            .sorted()
                            .rev()
                            .next()
                            .unwrap(),
                        Dir::Down => *grid
                            .keys()
                            .filter(|x| x.1 == pos.1)
                            .sorted()
                            .next()
                            .unwrap(),
                        Dir::Right => *grid
                            .keys()
                            .filter(|x| x.0 == pos.0)
                            .sorted()
                            .next()
                            .unwrap(),
                        Dir::Left => *grid
                            .keys()
                            .filter(|x| x.0 == pos.0)
                            .sorted()
                            .rev()
                            .next()
                            .unwrap(),
                    };
                    if let Some(Tile::Wall) = grid.get(&new_pos) {
                        break;
                    }
                    pos = new_pos;
                }
            }
            Instruction::TurnRight => {
                dir = dir.turned(instr);
            }
            Instruction::TurnLeft => {
                dir = dir.turned(instr);
            }
        }
    }
    let score = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir.score();
    dbg!(score);
    Ok(())
}

struct Quadrant {
    row_start: i64,
    col_start: i64,
    surrounding_quadrants: [(usize, usize); 4],
}

impl Quadrant {
    fn new(row_start: i64, col_start: i64, surrounding_quadrants: [(usize, usize); 4]) -> Self {
        Self {
            row_start,
            col_start,
            surrounding_quadrants,
        }
    }
}

fn next_pos(
    pos: (i64, i64),
    dir: Dir,
    quadrants: &[Quadrant],
    quadrant_size: i64,
) -> ((i64, i64), Dir) {
    let quadrant = quadrants
        .iter()
        .find(|q| {
            q.row_start / quadrant_size == pos.0 / quadrant_size
                && q.col_start / quadrant_size == pos.1 / quadrant_size
        })
        .unwrap();

    let delta = dir.delta();
    let (target_quadrant, rotations) = match dir {
        Dir::Up => quadrant.surrounding_quadrants[0],
        Dir::Left => quadrant.surrounding_quadrants[1],
        Dir::Down => quadrant.surrounding_quadrants[2],
        Dir::Right => quadrant.surrounding_quadrants[3],
    };
    let mut new_dir = dir;
    let mut new_relative_pos = (
        (pos.0 + delta.0).rem_euclid(quadrant_size),
        (pos.1 + delta.1).rem_euclid(quadrant_size),
    );
    assert!(
        new_relative_pos.0 == 0
            || new_relative_pos.0 == quadrant_size - 1
            || new_relative_pos.1 == 0
            || new_relative_pos.1 == quadrant_size - 1,
        "{:?} is not on the edge, orig pos {:?} and delta {:?} for dir {:?}?",
        new_relative_pos,
        pos,
        delta,
        dir
    );
    for _ in 0..rotations {
        new_relative_pos = (quadrant_size - new_relative_pos.1 - 1, new_relative_pos.0);
        new_dir = new_dir.turned(Instruction::TurnLeft);
    }

    let target_quadrant = &quadrants[target_quadrant];
    (
        (
            target_quadrant.row_start + new_relative_pos.0,
            target_quadrant.col_start + new_relative_pos.1,
        ),
        new_dir,
    )
}

fn part2(file_data: &str) -> Result<()> {
    let (_, (grid, instructions)) = parse_input(file_data).unwrap();

    let mut pos = *grid.keys().sorted_by_key(|x| (x.0, x.1)).next().unwrap();
    let mut dir = Dir::Right;

    let quadrants = vec![
        Quadrant::new(0, 50, [(5, 3), (3, 2), (2, 0), (1, 0)]),
        Quadrant::new(0, 100, [(5, 0), (0, 0), (2, 3), (4, 2)]),
        Quadrant::new(50, 50, [(0, 0), (3, 1), (4, 0), (1, 1)]),
        Quadrant::new(100, 0, [(2, 3), (0, 2), (5, 0), (4, 0)]),
        Quadrant::new(100, 50, [(2, 0), (3, 0), (5, 3), (1, 2)]),
        Quadrant::new(150, 0, [(3, 0), (0, 1), (1, 0), (4, 1)]),
    ];
    let quadrant_size = 50;

    for instr in instructions {
        match instr {
            Instruction::Forward(x) => {
                let mut delta = dir.delta();
                for _ in 0..x {
                    let test_pos = (pos.0 + delta.0, pos.1 + delta.1);
                    if let Some(Tile::Wall) = grid.get(&test_pos) {
                        break;
                    }
                    if let Some(Tile::Open) = grid.get(&test_pos) {
                        pos = test_pos;
                        continue;
                    }

                    let (test_pos, test_dir) = next_pos(pos, dir, &quadrants, quadrant_size);
                    if let Some(Tile::Wall) = grid.get(&test_pos) {
                        break;
                    }
                    if let Some(Tile::Open) = grid.get(&test_pos) {
                        (pos, dir) = (test_pos, test_dir);
                        delta = dir.delta();
                        continue;
                    }
                    // If we somehow end up _not_ on the existing grid, that's pretty bad...
                    unreachable!();
                }
            }
            Instruction::TurnRight => {
                dir = dir.turned(instr);
            }
            Instruction::TurnLeft => {
                dir = dir.turned(instr);
            }
        }
    }

    let score = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir.score();
    dbg!(score);
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
