//! Day 02
//!
//! Need to find a nicer "exactly one rule" Result<> for the rule table, else
//! this was fairly straightforward.

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

static RULE_TABLE: Lazy<Vec<(Move, Move, RoundResult)>> = Lazy::new(|| {
    Vec::from([
        (Move::Rock, Move::Rock, RoundResult::Draw),
        (Move::Rock, Move::Paper, RoundResult::Win),
        (Move::Rock, Move::Scissors, RoundResult::Loss),
        (Move::Paper, Move::Rock, RoundResult::Loss),
        (Move::Paper, Move::Paper, RoundResult::Draw),
        (Move::Paper, Move::Scissors, RoundResult::Win),
        (Move::Scissors, Move::Rock, RoundResult::Win),
        (Move::Scissors, Move::Paper, RoundResult::Loss),
        (Move::Scissors, Move::Scissors, RoundResult::Draw),
    ])
});

fn calc_result_score(result: RoundResult) -> i32 {
    match result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loss => 0,
    }
}

fn calc_move_score(output_move: Move) -> i32 {
    match output_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn part1(file_data: &str) -> Result<()> {
    let lines = file_data.lines();
    let mut total_score = 0;
    for (line_number, line) in lines.enumerate() {
        let (a, b) = line.split_once(' ').context("Couldn't parse line")?;
        println!("{a} {b}");
        let input_move = match a {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => bail!("Can't decode move"),
        };
        let output_move = match b {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => bail!("Can't decode move"),
        };
        let rule = RULE_TABLE
            .iter()
            .filter(|(i, o, _r)| i == &input_move && o == &output_move)
            .collect::<Vec<_>>();
        let result = match rule.as_slice() {
            &[x] => x.2,
            _ => bail!("Didn't find exactly one rule!"),
        };
        let move_score = calc_move_score(output_move);
        let result_score = calc_result_score(result);
        let round_score = move_score + result_score;
        println!("{line_number} {input_move:?} {output_move:?} {round_score} {move_score} {result_score}");
        total_score += round_score;
    }

    println!("{total_score}");
    Ok(())
}


fn part2(file_data: &str) -> Result<()> {
    let lines = file_data.lines();
    let mut total_score = 0;
    for (line_number, line) in lines.enumerate() {
        let (a, b) = line.split_once(' ').context("Couldn't parse line")?;
        println!("{a} {b}");
        let input_move = match a {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => bail!("Can't decode move"),
        };
        let result = match b {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => bail!("Can't decode move"),
        };
        let rule = RULE_TABLE
            .iter()
            .filter(|(i, _o, r)| i == &input_move && r == &result)
            .collect::<Vec<_>>();
        let output_move = match rule.as_slice() {
            &[x] => x.1,
            _ => bail!("Didn't find exactly one rule!"),
        };

        let move_score = calc_move_score(output_move);
        let result_score = calc_result_score(result);
        let round_score = move_score + result_score;
        println!("{line_number} {input_move:?} {output_move:?} {round_score} {move_score} {result_score}");
        total_score += round_score;
    }

    println!("{total_score}");
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
