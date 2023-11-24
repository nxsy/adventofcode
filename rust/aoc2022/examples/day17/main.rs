//! Day 17

use std::collections::BTreeSet;

use adventofcode2022::prelude::*;

const VERBOSE: bool = true;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Shape {
    Dash,
    Plus,
    BackwardsL,
    Pipe,
    Block,
}

impl Shape {
    fn width(&self) -> i32 {
        match self {
            Shape::Dash => 4,
            Shape::Plus => 3,
            Shape::BackwardsL => 3,
            Shape::Pipe => 1,
            Shape::Block => 2,
        }
    }

    fn height(&self) -> i32 {
        match self {
            Shape::Dash => 1,
            Shape::Plus => 3,
            Shape::BackwardsL => 3,
            Shape::Pipe => 4,
            Shape::Block => 2,
        }
    }
}

struct Shapes(HashMap<Shape, Vec<(i32, i32)>>, Vec<Shape>);

impl Shapes {
    fn new() -> Self {
        let mut shape_pieces = HashMap::new();
        shape_pieces.insert(Shape::Dash, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        shape_pieces.insert(Shape::Plus, vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]);
        shape_pieces.insert(
            Shape::BackwardsL,
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        );
        shape_pieces.insert(Shape::Pipe, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        shape_pieces.insert(Shape::Block, vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
        let shape_list = vec![
            Shape::Dash,
            Shape::Plus,
            Shape::BackwardsL,
            Shape::Pipe,
            Shape::Block,
        ];
        Shapes(shape_pieces, shape_list)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum MovementDir {
    Left,
    Right,
    Down,
}

impl From<char> for MovementDir {
    fn from(c: char) -> Self {
        match c {
            '<' => MovementDir::Left,
            '>' => MovementDir::Right,
            _ => unreachable!(),
        }
    }
}

struct Rock {
    shape: Shape,
    pos: (i32, i32),
}

impl Rock {
    fn do_move(
        &mut self,
        direction: MovementDir,
        shapes: &Shapes,
        chamber: &HashSet<(i32, i32)>,
    ) -> bool {
        if self.pos.0 == 0 && direction == MovementDir::Left {
            return false;
        }
        if self.pos.0 + self.shape.width() == 7 && direction == MovementDir::Right {
            return false;
        }
        if self.pos.1 == 0 && direction == MovementDir::Down {
            return false;
        }
        let delta = match direction {
            MovementDir::Left => (-1, 0),
            MovementDir::Right => (1, 0),
            MovementDir::Down => (0, -1),
        };
        for piece in &shapes.0[&self.shape] {
            let abs_pos = (
                self.pos.0 + piece.0 + delta.0,
                self.pos.1 + piece.1 + delta.1,
            );
            if chamber.contains(&abs_pos) {
                return false;
            }
        }
        self.pos = (self.pos.0 + delta.0, self.pos.1 + delta.1);
        true
    }
}

fn part1(file_data: &str) -> Result<()> {
    let shapes = Shapes::new();

    let jet_dirs = file_data.chars().map(MovementDir::from).collect::<Vec<_>>();

    let mut chamber = HashSet::new();
    let mut highest_rock = -1;
    let mut t = 0;
    for rock_number in 0..2022 {
        let mut rock = Rock {
            shape: shapes.1[rock_number % shapes.1.len()],
            pos: (2, highest_rock + 4),
        };
        if VERBOSE {
            print_stuff(&chamber, &rock, &shapes);
        }
        loop {
            let wind_dir = jet_dirs[t];
            t += 1;
            t %= jet_dirs.len();
            rock.do_move(wind_dir, &shapes, &chamber);
            if !rock.do_move(MovementDir::Down, &shapes, &chamber) {
                for piece in &shapes.0[&rock.shape] {
                    let abs_pos = (rock.pos.0 + piece.0, rock.pos.1 + piece.1);
                    highest_rock = highest_rock.max(abs_pos.1);
                    chamber.insert(abs_pos);
                }
                break;
            }
        }
    }
    dbg!(highest_rock);
    Ok(())
}

fn print_stuff(chamber: &HashSet<(i32, i32)>, rock: &Rock, shapes: &Shapes) {
    let height = rock.pos.1 + rock.shape.height();
    let mut pieces = HashSet::new();
    for piece in &shapes.0[&rock.shape] {
        pieces.insert((rock.pos.0 + piece.0, rock.pos.1 + piece.1));
    }
    for a in 0..height {
        let row = height - a - 1;
        print!("{row:4}|");
        for x in 0..7 {
            let pos = (x, row);
            if pieces.contains(&pos) {
                print!("@");
                continue;
            }
            if chamber.contains(&pos) {
                print!("#");
                continue;
            }
            print!(".");
        }
        print!("|");
        println!();
    }
    println!("    +-------+");
    println!();
}

fn part2(file_data: &str) -> Result<()> {
    let shapes = Shapes::new();

    let jet_dirs = file_data.chars().map(MovementDir::from).collect::<Vec<_>>();
    println!(
        "{} * {} = {}",
        jet_dirs.len(),
        shapes.1.len(),
        jet_dirs.len() * shapes.1.len()
    );
    let mut chamber = HashSet::new();
    let mut highest_rock = -1;

    let mut t = 0;
    let mut rock_number = 0;
    let mut seen = HashMap::new();

    let target = 1000000000000;
    let mut added_via_pattern = 0;
    while rock_number < target {
        let shape_number = rock_number % shapes.1.len();
        let mut rock = Rock {
            shape: shapes.1[shape_number],
            pos: (2, highest_rock + 4),
        };
        loop {
            let wind_dir = jet_dirs[t];
            t += 1;
            t %= jet_dirs.len();

            rock.do_move(wind_dir, &shapes, &chamber);
            if !rock.do_move(MovementDir::Down, &shapes, &chamber) {
                for piece in &shapes.0[&rock.shape] {
                    let abs_pos = (rock.pos.0 + piece.0, rock.pos.1 + piece.1);
                    highest_rock = highest_rock.max(abs_pos.1);
                    chamber.insert(abs_pos);
                }
                let top = chamber
                    .iter()
                    .filter_map(|(x, y)| {
                        if *y > highest_rock - 10 {
                            Some((highest_rock - *y, *x))
                        } else {
                            None
                        }
                    })
                    .collect::<BTreeSet<_>>();

                let key = (t, shape_number, top);
                if let Some((old_rock_number, old_highest_rock)) = seen.get(&key) {
                    let dy = (highest_rock as usize) - old_highest_rock;
                    let dr = rock_number - old_rock_number;
                    let repeats = (target - rock_number) / dr;
                    added_via_pattern += repeats * dy;
                    rock_number += repeats * dr;
                }
                seen.insert(key, (rock_number, highest_rock as usize));
                break;
            }
        }
        rock_number += 1;
    }
    dbg!(highest_rock as usize + added_via_pattern + 1);
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
