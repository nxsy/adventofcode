use std::collections::HashSet;
use std::fs::read_to_string;

use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
}

impl Default for Part {
    fn default() -> Self {
        Part::Part1
    }
}

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

impl Default for Input {
    fn default() -> Self {
        Input::ExampleInput
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", about = "Advent of Code 2021 Day 3")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug)]
struct Board {
    values: [u32; 25],
    winning_combos: Vec<HashSet<u32>>,
}

fn part1(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut lines = contents.lines();

    let numbers_drawn = lines.next().context("Missing initial numbers")?;

    let mut boards: Vec<Board> = Vec::new();

    while let Some(_blank) = lines.next() {
        let mut values: [u32; 25] = [0; 25];
        let mut winning_combos: Vec<HashSet<u32>> = Vec::new();
        for li in 0..5 {
            let line = lines.next().context("Partial board")?;
            let mut line_values: [u32; 5] = [0; 5];
            for (wi, sub) in line.split_whitespace().enumerate() {
                line_values[wi] = sub.parse::<u32>()?;
            }
            {
                let (_, right) = values.split_at_mut(li * 5);
                right[..5].copy_from_slice(&line_values);
            }
            winning_combos.push(HashSet::from(line_values));
        }

        for ci in 0..5 {
            let c_values_vec = values.iter().skip(ci).step_by(5).map(|x| x.to_owned());
            winning_combos.push(HashSet::from_iter(c_values_vec));
        }
        boards.push(Board {
            values,
            winning_combos,
        });
    }

    let mut winning_board: Option<&Board> = None;
    let mut losing_board: Option<&Board> = None;
    let mut winning_number: Option<u32> = None;
    let mut losing_number: Option<u32> = None;
    let mut winning_numbers_drawn_set: HashSet<u32> = HashSet::new();
    let mut numbers_drawn_set: HashSet<u32> = HashSet::new();
    let mut winning_boards: HashSet<usize> = HashSet::new();
    let all_boards: HashSet<usize> = HashSet::from_iter(0..boards.len());
    for number in numbers_drawn.split(',') {
        let number = number.parse::<u32>()?;
        losing_number = Some(number);
        if winning_board.is_none() {
            winning_numbers_drawn_set.insert(number);
        }
        numbers_drawn_set.insert(number);
        for (i, board) in boards.iter().enumerate() {
            if winning_boards.contains(&i) {
                continue;
            }
            for winning_combo in &board.winning_combos {
                if winning_combo.is_subset(&numbers_drawn_set) {
                    if winning_board.is_none() {
                        winning_board = Some(board);
                        winning_number = Some(number);
                    }
                    winning_boards.insert(i);
                }
            }
        }
        let losing_boards: Vec<_> = all_boards.difference(&winning_boards).collect();
        if losing_boards.len() == 1 {
            losing_board = Some(&boards[*losing_boards[0]]);
        }
        if losing_boards.is_empty() {
            break;
        }
    }

    let winning_board = winning_board.context("Could not find winning board")?;
    let winning_number = winning_number.context("Could not find winning number")?;
    let losing_board = losing_board.context("Could not find losing board")?;
    let losing_number = losing_number.context("Could not find losing number")?;

    {
        let board_values_set = HashSet::from(winning_board.values);
        let unmarked_values = board_values_set.difference(&winning_numbers_drawn_set);
        let unmarked_sum = unmarked_values.fold(0, |acc, v| acc + *v);

        println!("Winning board was: {:?}", winning_board);
        println!("Unmarked sum was {}", unmarked_sum);
        println!("Winning number was {}", winning_number);
        println!("Total is {}", unmarked_sum * winning_number);
    }

    {
        let board_values_set = HashSet::from(losing_board.values);
        let unmarked_values = board_values_set.difference(&numbers_drawn_set);
        let unmarked_sum = unmarked_values.fold(0, |acc, v| acc + *v);

        println!("Losing board was: {:?}", losing_board);
        println!("Unmarked sum was {}", unmarked_sum);
        println!("Losing number was {}", losing_number);
        println!("Total is {}", unmarked_sum * losing_number);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    // let args = Args { part: Part::Part1, input: Input::ExampleInput };
    println!("Args: {:#?}", args);

    let file_path = match args.input {
        Input::ExampleInput => "data/day4/example_input",
        Input::FinalInput => "data/day4/input",
    };
    match args.part {
        Part::Part1 => part1(file_path),
    }
}
