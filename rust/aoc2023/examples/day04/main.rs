use std::collections::BTreeSet;

use anyhow::Result;
use thiserror::Error;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32 as nom_u32},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug)]
struct Game {
    card_number: u32,
    card_numbers: BTreeSet<u32>,
    winning_numbers: BTreeSet<u32>,
}

impl Game {
    fn new(card_number: u32, card_numbers: BTreeSet<u32>, winning_numbers: BTreeSet<u32>) -> Self {
        Game {
            card_number,
            card_numbers,
            winning_numbers,
        }
    }
}

fn line_to_game(input: &str) -> IResult<&str, Game> {
    let (input, (card_number, (winning_numbers, card_numbers))) = tuple((
        delimited(
            tuple((tag("Card"), space1)),
            nom_u32,
            tuple((tag(":"), space1)),
        ),
        separated_pair(
            separated_list1(space1, nom_u32),
            tuple((space1, tag("|"), space1)),
            separated_list1(space1, nom_u32),
        ),
    ))(input)?;

    Ok((
        input,
        Game::new(
            card_number,
            card_numbers.iter().copied().collect::<BTreeSet<_>>(),
            winning_numbers.iter().copied().collect::<BTreeSet<_>>(),
        ),
    ))
}

fn part1(input: &str) -> Result<String> {
    let (_, games) = separated_list1(line_ending, line_to_game)(input).unwrap();
    let mut total = 0;
    for game in games {
        let common = game
            .card_numbers
            .intersection(&game.winning_numbers)
            .count() as u32;

        total += if common > 0 { 2u32.pow(common - 1) } else { 0 };
    }

    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String> {
    let (_, games) = separated_list1(line_ending, line_to_game)(input).unwrap();
    let mut total = 0;
    let mut extra_cards = vec![0; games.len()];

    for game in games {
        let common_count = game
            .card_numbers
            .intersection(&game.winning_numbers)
            .count();

        let this_card_number = game.card_number as usize - 1;
        let num_this_card = 1 + extra_cards[this_card_number];

        total += num_this_card;
        for i in 0..common_count {
            let extra_card_number = this_card_number + i + 1;
            *extra_cards.get_mut(extra_card_number).unwrap() += num_this_card;
        }
    }
    Ok(total.to_string())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let part1_result = match part1(input) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part1: {}", part1_result);
    let part2_result = match part2(input) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "13";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = 30;
        let actual = part2(file_data)?;
        assert_eq!(actual, expected.to_string());
        Ok(())
    }
}
