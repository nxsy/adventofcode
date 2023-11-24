// cargo run --example day21 -- (part1|part2) (example_input|final_input)

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

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day21", about = "Advent of Code 2021 Day 21")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    positions: [u32; 2],
    scores: [u32; 2],
    player: usize,
}

fn state_to_universes(state: State, memo: &mut HashMap<State, [u64; 2]>) -> [u64; 2] {
    let State {
        positions,
        scores,
        player,
    } = state;
    if memo.contains_key(&state) {
        return memo[&state];
    }
    let wins = if scores[0] >= 21 {
        [1, 0]
    } else if scores[1] >= 21 {
        [0, 1]
    } else {
        let mut wins = [0, 0];
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let new_pos = (positions[player] + i + j + k) % 10;
                    let score = if new_pos == 0 { 10 } else { new_pos };

                    let mut positions = positions;
                    positions[player] = new_pos;

                    let mut scores = scores;
                    scores[player] += score;

                    let this_wins = state_to_universes(
                        State {
                            positions,
                            scores,
                            player: 1 - player,
                        },
                        memo,
                    );

                    wins[0] += this_wins[0];
                    wins[1] += this_wins[1];
                }
            }
        }

        wins
    };
    memo.insert(state, wins);
    wins
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;
    let mut lines = contents.lines();
    let original_positions = [
        lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap(),
        lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap(),
    ];
    let mut positions = original_positions;

    let mut scores = [0, 0];
    let mut next_player = 0;
    let mut dice_rolls = (1..=100).cycle();
    let mut num_dice_rolls = 0;
    while scores[0].max(scores[1]) < 1000 {
        num_dice_rolls += 3;
        let roll =
            dice_rolls.next().unwrap() + dice_rolls.next().unwrap() + dice_rolls.next().unwrap();
        let new_pos = (positions[next_player] + roll) % 10;
        positions[next_player] = new_pos;
        scores[next_player] += if new_pos == 0 { 10 } else { new_pos };
        next_player = (next_player + 1) % 2;
    }

    println!("Part 1: {}", num_dice_rolls * scores[0].min(scores[1]));

    let state = State {
        positions: original_positions,
        scores: [0, 0],
        player: 0,
    };
    let mut memo = HashMap::new();
    let wins = state_to_universes(state, &mut memo);

    println!("Part 2: {:?}", wins);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day21/example_input",
        Input::FinalInput => "data/day21/input",
    };

    solve(file_path)
}
