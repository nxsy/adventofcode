//! Day 05
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

#[derive(Default)]
struct Stacks(Vec<Vec<char>>);

impl From<&str> for Stacks {
    fn from(prelude: &str) -> Self {
        let mut res = Stacks::default();
        let prelude = prelude.lines().collect::<Vec<_>>();
        let highest_stack = prelude.len() - 1;
        let num_stacks = prelude.last().unwrap().len() / 4 + 1;
        for _ in 0..num_stacks {
            res.0.push(Vec::new());
        }
        for i in 0..highest_stack {
            let l = prelude
                .get(highest_stack - i - 1)
                .unwrap()
                .chars()
                .collect::<Vec<_>>();
            for (j, stack) in res.0.iter_mut().enumerate() {
                let p = 1 + j * 4;
                let x = l[p];
                if x != ' ' {
                    stack.push(x);
                }
            }
        }
        res
    }
}

fn part1(file_data: &str) -> Result<()> {
    let (prelude, moves) = file_data
        .split_once("\r\n\r\n")
        .context("Couldn't split into prelude and moves")?;

    let mut stacks = Stacks::from(prelude);

    for m in moves.lines() {
        let m_ws = m
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();
        let (num, from, to) = (m_ws[0], m_ws[1], m_ws[2]);
        for _ in 0..num {
            let x = stacks
                .0
                .get_mut((from - 1) as usize)
                .unwrap()
                .pop()
                .unwrap();
            stacks.0.get_mut((to - 1) as usize).unwrap().push(x);
        }
    }

    for mut stack in stacks.0 {
        let a = stack.pop();
        if let Some(a) = a {
            print!("{a}");
        }
    }
    println!();
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let (prelude, moves) = file_data
        .split_once("\r\n\r\n")
        .context("Couldn't split into prelude and moves")?;
    let mut stacks = Stacks::from(prelude);

    for m in moves.lines() {
        let m_ws = m
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();
        let (num, from, to) = (m_ws[0], m_ws[1], m_ws[2]);
        let mut s = VecDeque::new();
        for _ in 0..num {
            s.push_front(
                stacks
                    .0
                    .get_mut((from - 1) as usize)
                    .unwrap()
                    .pop()
                    .unwrap(),
            );
        }
        let to_stack = stacks.0.get_mut((to - 1) as usize).unwrap();
        for str in s {
            to_stack.push(str);
        }
    }

    for mut stack in stacks.0 {
        let a = stack.pop();
        if let Some(a) = a {
            print!("{a}");
        }
    }
    println!();
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
