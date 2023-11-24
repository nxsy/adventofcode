//! Day 09
//!
//! This was fun.

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
    ExampleInput2,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    part: Part,
    #[arg(long)]
    input: Input,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unimplemented!(),
        }
    }
}

struct Command {
    dir: Direction,
    distance: usize,
}

impl From<&str> for Command {
    fn from(line: &str) -> Self {
        let (a, b) = line.split_once(' ').unwrap();
        Command {
            dir: Direction::from(a),
            distance: b.parse().unwrap(),
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos(i32, i32);
impl Pos {
    fn go(&mut self, dir: Direction) {
        let (dx, dy) = match dir {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        self.0 += dx;
        self.1 += dy;
    }
}

struct Rope {
    pos: Vec<Pos>,
    tail_visited: HashSet<Pos>,
    bounds: (i32, i32, i32, i32),
}

impl Rope {
    fn move_head(&mut self, dir: Direction, distance: usize) {
        for _ in 0..distance {
            self.pos[0].go(dir);
            self.bounds = (
                self.bounds.0.min(self.pos[0].0),
                self.bounds.1.min(self.pos[0].1),
                self.bounds.2.max(self.pos[0].0),
                self.bounds.3.max(self.pos[0].1),
            );
            for knot in 1..self.pos.len() {
                let parent_pos = self.pos[knot - 1];
                let knot_pos = &mut self.pos[knot];
                let relative_pos = (parent_pos.0 - knot_pos.0, parent_pos.1 - knot_pos.1);
                match relative_pos {
                    (x, y) if x.abs() <= 1 && y.abs() <= 1 => {}
                    // (0, y) => tail_pos.1 += y,
                    // (x, 0) => tail_pos.0 += x,
                    (x, y) => {
                        knot_pos.0 += x.signum();
                        knot_pos.1 += y.signum();
                    }
                }
            }
            let tail_pos = self.pos[self.pos.len() - 1];
            self.tail_visited.insert(tail_pos);
            // let head_pos = self.pos[0];
            // println!("H: {head_pos:?}, T: {tail_pos:?}");
        }
    }

    fn draw(&self) {
        for row in self.bounds.1..=self.bounds.3 {
            for col in self.bounds.0..=self.bounds.2 {
                let p = Pos(col, 4 - row);
                print!(
                    "{}",
                    if self.tail_visited.contains(&p) {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
    }

    fn new(size: usize) -> Self {
        Self {
            pos: vec![Pos::default(); size],
            tail_visited: HashSet::new(),
            bounds: (0, 0, 0, 0),
        }
    }
}

fn part1(file_data: &str) -> Result<()> {
    let instructions = file_data.lines().map(Command::from);
    let mut rope = Rope::new(2);

    for Command { dir, distance } in instructions {
        rope.move_head(dir, distance);
    }

    rope.draw();

    dbg!(rope.tail_visited.len());
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let instructions = file_data.lines().map(Command::from);
    let mut rope = Rope::new(10);

    for Command { dir, distance } in instructions {
        rope.move_head(dir, distance);
    }

    rope.draw();

    dbg!(rope.tail_visited.len());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::ExampleInput => include_str!("example_input"),
        Input::FinalInput => include_str!("input"),
        Input::ExampleInput2 => include_str!("example_input2"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
