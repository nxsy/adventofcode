use std::collections::HashMap;

use anyhow::Result;
use nom::{
    character::complete::{line_ending, one_of, space1, u32 as nom_u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardValue {
    _Joker,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _T,
    _J,
    _Q,
    _K,
    _A,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<CardValue>,
    bid: u32,
}

impl Hand {
    fn score(&self) -> HandType {
        let joker_count = self
            .cards
            .iter()
            .filter(|card| **card == CardValue::_Joker)
            .count();

        let mut card_counts: HashMap<CardValue, i32> =
            self.cards.iter().fold(HashMap::new(), |mut acc, card| {
                if card != &CardValue::_Joker {
                    acc.entry(*card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                acc
            });

        let mut card_counts: Vec<_> = card_counts.drain().collect();
        card_counts.sort_by_key(|(_, count)| -*count);

        if card_counts.is_empty() {
            // All jokers
            return HandType::FiveOfAKind;
        }

        card_counts[0].1 += joker_count as i32;
        if card_counts.len() == 1 {
            HandType::FiveOfAKind
        } else if card_counts.len() == 2 {
            if card_counts[0].1 == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if card_counts.len() == 3 {
            if card_counts[0].1 == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if card_counts.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

fn parse_card_value(input: &str) -> IResult<&str, CardValue> {
    let (input, card) = one_of("23456789TJQKA")(input)?;
    let card = match card {
        '2' => CardValue::_2,
        '3' => CardValue::_3,
        '4' => CardValue::_4,
        '5' => CardValue::_5,
        '6' => CardValue::_6,
        '7' => CardValue::_7,
        '8' => CardValue::_8,
        '9' => CardValue::_9,
        'T' => CardValue::_T,
        'J' => CardValue::_J,
        'Q' => CardValue::_Q,
        'K' => CardValue::_K,
        'A' => CardValue::_A,
        _ => unreachable!(),
    };
    Ok((input, card))
}

fn parse_hand(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(
        line_ending,
        separated_pair(many1(parse_card_value), space1, nom_u32),
    )(input)?;
    Ok((
        input,
        hands
            .into_iter()
            .map(|(cards, bid)| Hand { cards, bid })
            .collect(),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandWithScore(Hand, HandType);

impl PartialOrd for HandWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.1.cmp(&other.1) {
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

fn part1(input: &'static str) -> Result<String> {
    let (_, hand) = parse_hand(input).unwrap();
    let mut hands_with_scores: Vec<_> = hand
        .iter()
        .map(|hand| HandWithScore(hand.clone(), hand.score()))
        .collect();
    hands_with_scores.sort();
    let total: usize = hands_with_scores
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.0.bid as usize)
        .sum();
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String> {
    let (_, mut hands) = parse_hand(input).unwrap();
    for hand in hands.iter_mut() {
        for card in hand.cards.iter_mut() {
            if *card == CardValue::_J {
                *card = CardValue::_Joker;
            }
        }
    }
    let mut hands_with_scores: Vec<_> = hands
        .iter()
        .map(|hand| HandWithScore(hand.clone(), hand.score()))
        .collect();
    hands_with_scores.sort();
    let total: usize = hands_with_scores
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.0.bid as usize)
        .sum();
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
        let expected = "6440";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "5905";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
