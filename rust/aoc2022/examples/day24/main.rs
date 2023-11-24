//! Day 22

use std::cmp::Reverse;

use adventofcode2022::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{many1, separated_list1},
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
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    #[allow(clippy::inherent_to_string)]
    fn to_string(self) -> String {
        match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
        }
        .to_string()
    }

    fn delta(&self) -> Pos {
        match self {
            Direction::Up => Pos { row: -1, col: 0 },
            Direction::Right => Pos { row: 0, col: 1 },
            Direction::Down => Pos { row: 1, col: 0 },
            Direction::Left => Pos { row: 0, col: -1 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Blizzard(Direction),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> usize {
        ((self.row - other.row).abs() + (self.col - other.col).abs()) as usize
    }
}

fn parse_map_line(input: &str) -> IResult<&str, HashMap<i64, Tile>> {
    let (input, chars) = many1(alt((
        tag("."),
        tag("#"),
        tag(">"),
        tag("<"),
        tag("^"),
        tag("v"),
    )))(input)?;
    let mut h = HashMap::new();
    for (i, c) in chars.into_iter().enumerate() {
        if let Some(tile) = match c {
            "." => Some(Tile::Open),
            "#" => Some(Tile::Wall),
            ">" => Some(Tile::Blizzard(Direction::Right)),
            "<" => Some(Tile::Blizzard(Direction::Left)),
            "^" => Some(Tile::Blizzard(Direction::Up)),
            "v" => Some(Tile::Blizzard(Direction::Down)),
            _ => unreachable!(),
        } {
            h.insert(i as i64, tile);
        }
    }
    Ok((input, h))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<Pos, Tile>> {
    let (input, map_tiles) = separated_list1(line_ending, parse_map_line)(input)?;
    let mut h = HashMap::new();
    for (i, l) in map_tiles.into_iter().enumerate() {
        for (k, v) in l {
            h.insert(
                Pos {
                    row: i as i64,
                    col: k,
                },
                v,
            );
        }
    }
    Ok((input, h))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Delta(i64, i64);

fn print_map(
    walls: &HashSet<Pos>,
    blizzards: &Vec<(Pos, Direction)>,
    bottom_right: Pos,
    player: Pos,
) {
    println!();
    let mut blizzards_count: HashMap<Pos, i64> = HashMap::new();
    let mut blizzards_hash = HashMap::new();
    for &b in blizzards {
        *blizzards_count.entry(b.0).or_default() += 1;
        blizzards_hash.insert(b.0, b.1.to_string());
    }
    for r in 0..=bottom_right.row {
        for c in 0..=bottom_right.col {
            let p = Pos { row: r, col: c };
            if p == player {
                print!("E");
            } else if walls.contains(&p) {
                print!("#");
            } else if let Some(c) = blizzards_count.get(&p) {
                if *c == 1 {
                    print!("{}", blizzards_hash[&p]);
                } else {
                    print!("{}", c);
                }
            } else {
                print!(".");
            };
        }
        println!();
    }
    println!();
}

struct Field {
    state: Vec<Vec<(Pos, Direction)>>,
    sets: Vec<HashSet<Pos>>,
    walls: HashSet<Pos>,
    bounds: [Pos; 2],
    start_pos: Pos,
    end_pos: Pos,
}

impl Field {
    fn new(blizzards: Vec<(Pos, Direction)>, walls: HashSet<Pos>) -> Self {
        let sets = vec![blizzards.iter().map(|x| x.0).collect::<HashSet<_>>()];
        let state = vec![blizzards];

        let row_end = walls.iter().map(|p| p.row).sorted().rev().next().unwrap();
        let col_end = walls.iter().map(|p| p.col).sorted().rev().next().unwrap();

        let top_left = Pos { row: 0, col: 0 };
        let start_pos = Pos { row: 0, col: 1 };
        let end_pos = Pos {
            row: row_end,
            col: col_end - 1,
        };
        let bottom_right = Pos {
            row: row_end,
            col: col_end,
        };

        let bounds = [top_left, bottom_right];

        Self {
            state,
            sets,
            walls,
            bounds,
            start_pos,
            end_pos,
        }
    }

    fn state_at(&mut self, round: usize) -> (&Vec<(Pos, Direction)>, &HashSet<Pos>) {
        while self.state.len() - 1 < round {
            let last_round = self.state.len() - 1;
            let mut blizzards = self.state[last_round].clone();

            for (p, d) in blizzards.iter_mut() {
                let delta = d.delta();
                *p = Pos {
                    row: p.row + delta.row,
                    col: p.col + delta.col,
                };
                if self.walls.contains(p) {
                    match d {
                        Direction::Up => p.row = self.bounds[1].row - 1,
                        Direction::Right => p.col = self.bounds[0].col + 1,
                        Direction::Down => p.row = self.bounds[0].row + 1,
                        Direction::Left => p.col = self.bounds[1].col - 1,
                    }
                }
            }
            let sets = blizzards.iter().map(|x| x.0).collect::<HashSet<_>>();
            self.state.push(blizzards);
            self.sets.push(sets);
        }
        (&self.state[round], &self.sets[round])
    }

    #[allow(unused)]
    fn print_map(&self, round: usize, player: Pos) {
        if round > self.state.len() - 1 {
            unreachable!();
        }

        let blizzards = &self.state[round];
        print_map(
            &self.walls,
            blizzards,
            self.bounds[1],
            player,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    cost: Reverse<usize>,
    round: Reverse<usize>,
    distance: Reverse<usize>,
    pos: Pos,
}

fn part1(file_data: &str) -> Result<()> {
    let mut field = field_from_input(file_data);    
    let start_round = 0;
    let start_pos = field.start_pos;
    let end_pos = field.end_pos;
    let winner = run_once(&mut field, start_round, start_pos, end_pos);
    dbg!(winner);

    Ok(())
}

fn run_once(field: &mut Field, start_round: usize, start_pos: Pos, end_pos: Pos) -> Option<State> {
    let mut winner = None;
    let mut stack = BinaryHeap::new();
    let manhattan_distance = start_pos.manhattan_distance(&end_pos);
    stack.push(State {
        cost: Reverse(start_round + manhattan_distance),
        round: Reverse(start_round),
        distance: Reverse(manhattan_distance),
        pos: start_pos,
    });
    let mut seen = HashSet::new();
    while let Some(s) = stack.pop() {
        if seen.contains(&s) {
            continue;
        }
        seen.insert(s);
        let (_, blizzard_set) = field.state_at(s.round.0 + 1);
        let blizzard_set = blizzard_set.clone();
        if s.distance.0 == 0 {
            winner = Some(s);
            break;
        }
        for d in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            let dpos = d.delta();
            let pos = Pos {
                row: s.pos.row + dpos.row,
                col: s.pos.col + dpos.col,
            };
            if pos.row < field.bounds[0].row
                || pos.row > field.bounds[1].row
                || pos.col < field.bounds[0].col
                || pos.col > field.bounds[1].col
            {
                continue;
            }
            if field.walls.contains(&pos) {
                continue;
            }
            if blizzard_set.contains(&pos) {
                // next round will have the blizzard
                continue;
            }
            let manhattan_distance = pos.manhattan_distance(&end_pos);
            stack.push(State {
                cost: Reverse(s.round.0 + 1 + manhattan_distance),
                round: Reverse(s.round.0 + 1),
                distance: Reverse(manhattan_distance),
                pos,
            })
        }
        if blizzard_set.contains(&s.pos) {
            continue;
        }
        stack.push(State {
            cost: Reverse(s.round.0 + 1 + s.distance.0),
            round: Reverse(s.round.0 + 1),
            distance: s.distance,
            pos: s.pos,
        })
    }
    winner
}

fn field_from_input(file_data: &str) -> Field {
    let (_, grid) = all_consuming(parse_input)(file_data).unwrap();
    let blizzards = grid
        .iter()
        .filter_map(|(&p, &t)| {
            if let Tile::Blizzard(d) = t {
                Some((p, d))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let walls = grid
        .iter()
        .filter_map(|(&p, &t)| if let Tile::Wall = t { Some(p) } else { None })
        .collect::<HashSet<_>>();
    
    Field::new(blizzards, walls)
}

fn part2(file_data: &str) -> Result<()> {
    let mut field = field_from_input(file_data);    
    let start_round = 0;
    let start_pos = field.start_pos;
    let end_pos = field.end_pos;
    let winner = run_once(&mut field, start_round, start_pos, end_pos).unwrap();
    println!("{}", winner.round.0);
    let winner = run_once(&mut field, winner.round.0, end_pos, start_pos).unwrap();
    println!("{}", winner.round.0);
    let winner = run_once(&mut field, winner.round.0, start_pos, end_pos).unwrap();
    println!("{}", winner.round.0);

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
