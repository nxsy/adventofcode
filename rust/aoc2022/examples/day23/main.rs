//! Day 22

use adventofcode2022::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    IResult,
};

const VERBOSE: bool = false;

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
    ExampleInput2,
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
    Elf,
}

fn parse_map_line(input: &str) -> IResult<&str, HashMap<i64, Tile>> {
    let (input, chars) = many1(alt((tag("."), tag("#"))))(input)?;
    let mut h = HashMap::new();
    for (i, c) in chars.into_iter().enumerate() {
        if let Some(tile) = match c {
            "." => Some(Tile::Open),
            "#" => Some(Tile::Elf),
            _ => unreachable!(),
        } {
            h.insert(i as i64, tile);
        }
    }
    Ok((input, h))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<(i64, i64), Tile>> {
    let (input, map_tiles) = separated_list1(line_ending, parse_map_line)(input)?;
    let mut h = HashMap::new();
    for (i, l) in map_tiles.into_iter().enumerate() {
        for (k, v) in l {
            h.insert((i as i64, k), v);
        }
    }
    Ok((input, h))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Delta(i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Direction {
    fn from_delta(delta: Delta) -> Self {
        match delta {
            Delta(-1, 0) => Direction::North,
            Delta(-1, 1) => Direction::NorthEast,
            Delta(0, 1) => Direction::East,
            Delta(1, 1) => Direction::SouthEast,
            Delta(1, 0) => Direction::South,
            Delta(1, -1) => Direction::SouthWest,
            Delta(0, -1) => Direction::West,
            Delta(-1, -1) => Direction::NorthWest,
            _ => unreachable!(),
        }
    }
}

impl Pos {
    fn surrounding_pos(&self) -> Vec<(Pos, Direction, Delta)> {
        let mut res = Vec::new();
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let delta = Delta(dr, dc);
                let direction = Direction::from_delta(delta);
                let new_pos = Pos(self.0 + delta.0, self.1 + delta.1);
                res.push((new_pos, direction, delta))
            }
        }
        res
    }
}

enum ProposedMove {
    Blocked, // some other elf was going to go there,
    MoveFrom(Pos),
}

fn print_map(elves: &HashSet<Pos>) {
    let row_start = elves.iter().map(|p| p.0).sorted().next().unwrap();
    let row_end = elves.iter().map(|p| p.0).sorted().rev().next().unwrap();
    let col_start = elves.iter().map(|p| p.1).sorted().next().unwrap();
    let col_end = elves.iter().map(|p| p.1).sorted().rev().next().unwrap();

    println!();
    for r in row_start - 1..=row_end + 1 {
        for c in col_start - 1..=col_end + 1 {
            if elves.contains(&Pos(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1(file_data: &str) -> Result<()> {
    let (_, grid) = all_consuming(parse_input)(file_data).unwrap();

    let (elves, _) = run(grid, Some(10));

    let row_start = elves.iter().map(|p| p.0).sorted().next().unwrap();
    let row_end = elves.iter().map(|p| p.0).sorted().rev().next().unwrap();
    let col_start = elves.iter().map(|p| p.1).sorted().next().unwrap();
    let col_end = elves.iter().map(|p| p.1).sorted().rev().next().unwrap();

    let rectangle = (row_end - row_start + 1) * (col_end - col_start + 1);
    dbg!(rectangle);
    let score = rectangle - elves.len() as i64;
    dbg!(score);
    Ok(())
}

fn run(grid: HashMap<(i64, i64), Tile>, rounds: Option<usize>) -> (HashSet<Pos>, usize) {
    let mut elves = grid
        .into_iter()
        .filter_map(|(p, c)| {
            if c == Tile::Elf {
                Some(Pos(p.0, p.1))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    let rules = [
        (Direction::North, Direction::NorthEast, Direction::NorthWest),
        (Direction::South, Direction::SouthEast, Direction::SouthWest),
        (Direction::West, Direction::NorthWest, Direction::SouthWest),
        (Direction::East, Direction::SouthEast, Direction::NorthEast),
    ];
    if VERBOSE {
        println!("== Initial state == ({})\n", elves.len());
        print_map(&elves);
    }
    for round in 0..rounds.unwrap_or(usize::MAX) {
        let mut proposed_moves = HashMap::new();
        for &elf in &elves {
            let mut elf_directions = HashMap::new();
            let surrounding_pos = elf.surrounding_pos();
            for (p, d, _) in &surrounding_pos {
                if elves.contains(p) {
                    elf_directions.insert(*d, *p);
                }
            }
            if !elf_directions.is_empty() {
                for i in 0..4 {
                    let (d1, d2, d3) = rules[((round + i) % 4)];
                    if elf_directions.contains_key(&d1)
                        || elf_directions.contains_key(&d2)
                        || elf_directions.contains_key(&d3)
                    {
                        continue;
                    }
                    let &(target_pos, ..) =
                        surrounding_pos.iter().find(|(_, d, _)| *d == d1).unwrap();
                    match proposed_moves.entry(target_pos) {
                        std::collections::hash_map::Entry::Occupied(mut e) => {
                            e.insert(ProposedMove::Blocked);
                        }
                        std::collections::hash_map::Entry::Vacant(e) => {
                            e.insert(ProposedMove::MoveFrom(elf));
                        }
                    }
                    break;
                }
            }
        }
        let mut any_move = false;
        for (p, m) in proposed_moves {
            if let ProposedMove::MoveFrom(from_pos) = m {
                elves.remove(&from_pos);
                assert!(!elves.contains(&p));
                elves.insert(p);
                any_move = true;
            }
        }
        if !any_move {
            return (elves, round + 1);
        }
        if VERBOSE {
            println!("== End of round {} == ({})\n", round, elves.len());
            print_map(&elves);
        }
    }
    (elves, rounds.unwrap())
}

fn part2(file_data: &str) -> Result<()> {
    let (_, grid) = all_consuming(parse_input)(file_data).unwrap();

    let (_, rounds) = run(grid, None);

    dbg!(rounds);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::ExampleInput => include_str!("example_input"),
        Input::ExampleInput2 => include_str!("example_input2"),
        Input::FinalInput => include_str!("input"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
