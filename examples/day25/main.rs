// cargo run --example day25 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
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
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day25", about = "Advent of Code 2021 Day 25")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Copy, Clone)]
enum Cucumber {
    East,
    South,
}

#[derive(Clone, Default)]
struct Grid {
    size: (usize, usize),
    elements: HashMap<(usize, usize), Cucumber>,
}

fn serialize_grid(grid: &Grid) -> String {
    let mut grid_str = String::new();
    for y in 0..grid.size.1 {
        for x in 0..grid.size.0 {
            let e = grid.elements.get(&(x, y));
            let c = match e {
                Some(&Cucumber::East) => '>',
                Some(&Cucumber::South) => 'v',
                None => '.',
            };
            grid_str.push(c);
        }
        grid_str.push('\n');
    }
    grid_str
}

fn one_move(grid: &Grid) -> (Grid, usize) {
    let mut moves = 0;
    let mut new_grid = Grid {
        size: grid.size,
        ..Default::default()
    };
    for (&(x, y), &cucumber) in &grid.elements {
        if let Cucumber::East = cucumber {
            let moved_pos = ((x + 1) % grid.size.0, y);
            if grid.elements.get(&moved_pos).is_none() {
                moves += 1;
                new_grid.elements.insert(moved_pos, cucumber);
            } else {
                new_grid.elements.insert((x, y), cucumber);
            }
        } else if let Cucumber::South = cucumber {
            new_grid.elements.insert((x, y), cucumber);
        }
    }
    let grid = new_grid;
    let mut new_grid = Grid {
        size: grid.size,
        ..Default::default()
    };
    for (&(x, y), &cucumber) in &grid.elements {
        if let Cucumber::South = cucumber {
            let moved_pos = (x, (y + 1) % grid.size.1);
            if grid.elements.get(&moved_pos).is_none() {
                moves += 1;
                new_grid.elements.insert(moved_pos, cucumber);
            } else {
                new_grid.elements.insert((x, y), cucumber);
            }
        } else if let Cucumber::East = cucumber {
            new_grid.elements.insert((x, y), cucumber);
        }
    }

    (new_grid, moves)
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let grid = load_grid(&contents);
    serialize_grid(&grid);

    let part1_moves = (|| {
        let mut grid = grid.clone();
        for i in 0.. {
            let (new_grid, moves) = one_move(&grid);
            grid = new_grid;
            if moves == 0 {
                return i + 1;
            }
        }
        unreachable!();
    })();

    println!("Moves: {}", part1_moves);

    Ok(())
}

fn load_grid(contents: &str) -> Grid {
    let mut elements = HashMap::new();
    let mut size = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        let line = line.trim();
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => {
                    elements.insert((x, y), Cucumber::East);
                }
                'v' => {
                    elements.insert((x, y), Cucumber::South);
                }
                _ => {}
            }
            size.0 = size.0.max(x + 1);
        }
        size.1 = size.1.max(y + 1);
    }

    Grid { size, elements }
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day25/example_input",
        Input::FinalInput => "data/day25/input",
    };

    solve(file_path)
}

#[cfg(test)]
mod tests {
    use crate::{load_grid, one_move, serialize_grid};

    fn trim_multiline_str(s: &str) -> String {
        let mut new_str = String::new();
        for l in s.lines() {
            let l = l.trim();
            new_str.push_str(l);
            new_str.push('\n');
        }
        new_str
    }

    #[test]
    fn test_example_1() {
        let initial = trim_multiline_str(
            "...>...
             .......
             ......>
             v.....>
             ......>
             .......
             ..vvv..",
        );

        let initial_grid = load_grid(&initial);
        assert_eq!(initial, serialize_grid(&initial_grid));

        let (step1_grid, _) = one_move(&initial_grid);
        let step1 = trim_multiline_str(
            "..vv>..
             .......
             >......
             v.....>
             >......
             .......
             ....v..",
        );
        assert_eq!(step1, serialize_grid(&step1_grid));
        let (step2_grid, _) = one_move(&step1_grid);
        let step2 = trim_multiline_str(
            "....v>.
             ..vv...
             .>.....
             ......>
             v>.....
             .......
             .......",
        );
        assert_eq!(step2, serialize_grid(&step2_grid));
    }
}
