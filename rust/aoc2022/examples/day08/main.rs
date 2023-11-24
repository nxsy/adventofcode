//! Day 08
//!
//!

use adventofcode2022::prelude::*;

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
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

fn part1(file_data: &str) -> Result<()> {
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for (row, line) in file_data.lines().enumerate() {
        rows += 1;
        for (col, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            if row == 0 {
                cols += 1;
            }
            grid.insert((row, col), c as usize);
        }
    }

    let mut searches: Vec<(usize, usize, i32, i32)> = Vec::new();
    for row in 0..rows {
        searches.push((row, 0, 0, 1));
        searches.push((row, cols - 1, 0, -1));
    }
    for col in 0..cols {
        searches.push((0, col, 1, 0));
        searches.push((rows - 1, col, -1, 0));
    }

    for (row, col, dr, dc) in searches {
        let mut tallest_seen = None;
        let mut r = row;
        let mut c = col;
        loop {
            match grid.get(&(r, c)) {
                None => break,
                Some(h) => match (h, tallest_seen) {
                    (h, Some(tallest_so_far)) if *h > tallest_so_far => {
                        tallest_seen = Some(*h);
                        visible.insert((r, c));
                    }
                    (h, None) => {
                        tallest_seen = Some(*h);
                        visible.insert((r, c));
                    }
                    _ => {}
                },
            }
            r = ((r as i32) + dr) as usize;
            c = ((c as i32) + dc) as usize;
        }
    }

    println!("{}", visible.len());
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    for (row, line) in file_data.lines().enumerate() {
        rows += 1;
        for (col, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            if row == 0 {
                cols += 1;
            }
            grid.insert((row, col), c as usize);
        }
    }

    let mut grid_score: HashMap<(usize, usize), usize> = HashMap::new();
    for row in 0..rows {
        for col in 0..cols {
            let initial_height = match grid.get(&(row, col)) {
                Some(h) => *h,
                None => unreachable!(),
            };
            let mut score = 1;
            for (dr, dc) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
            ] {
                let mut seen = 0;
                let mut r = row;
                let mut c = col;

                loop {
                    r = ((r as i32) + dr) as usize;
                    c = ((c as i32) + dc) as usize;
                    match grid.get(&(r, c)) {
                        None => break,
                        Some(h) => {
                            seen += 1;
                            if *h >= initial_height {
                                break;
                            }
                        },
                    }
                }
                score *= seen;
            }
            grid_score.insert((row, col), score);
        }
    }

    let mut best = None;
    for row in 0..rows {
        for col in 0..cols {
            let score = match grid_score.get(&(row, col)) {
                Some(score) => *score,
                None => unreachable!(),
            };
            match best {
                Some((_, _, other_score)) => {
                    if other_score < score {
                        best = Some((row, col, score))
                    }
                },
                None => best = Some((row, col, score)),
            }
        }
    }
    dbg!(best);
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
