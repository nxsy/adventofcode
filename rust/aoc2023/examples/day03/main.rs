use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

struct Board {
    symbols: BTreeMap<(i32, i32), (char, BTreeSet<u32>)>,
    span_numbers: BTreeMap<u32, u32>,
}

fn parse_board(input: &str) -> Board {
    let mut numbers = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    let mut span_id: u32 = 0;

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, char) in line.char_indices() {
            let x = x as i32;
            if let Some(n) = char.to_digit(10) {
                let this_span_id = if let Some((_, prev_span_id)) = numbers.get(&(x - 1, y)) {
                    *prev_span_id
                } else {
                    span_id += 1;
                    span_id
                };
                numbers.insert((x, y), (n, this_span_id));
            } else if char != '.' {
                symbols.insert((x, y), (char, BTreeSet::new()));
            }
        }
    }

    for ((x, y), (_, seen)) in symbols.iter_mut() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some((_, span_id)) = numbers.get(&(x + dx, y + dy)) {
                    seen.insert(*span_id);
                }
            }
        }
    }

    let mut span_numbers = BTreeMap::new();
    for (_, &(n, span_id)) in numbers.iter() {
        span_numbers
            .entry(span_id)
            .and_modify(|sn| *sn = *sn * 10 + n)
            .or_insert(n);
    }

    Board {
        symbols,
        span_numbers,
    }
}

fn part1(input: &str) -> Result<String> {
    let board = parse_board(input);

    let mut seen_spans = BTreeSet::new();
    for (_, (_, seen)) in board.symbols.iter() {
        seen_spans.extend(seen);
    }

    let mut span_numbers = Vec::new();

    for span_id in seen_spans.iter() {
        span_numbers.push(board.span_numbers[span_id]);
    }

    Ok(span_numbers.iter().sum::<u32>().to_string())
}

fn part2(input: &str) -> Result<String> {
    let board = parse_board(input);

    let gears = board
        .symbols
        .iter()
        .filter_map(|(_, (c, seen))| (*c == '*' && seen.len() == 2).then_some(seen.clone()))
        .collect::<Vec<_>>();

    let mut spams_sum = 0;
    for spans in gears.iter() {
        let span_product = spans
            .iter()
            .map(|span_id| board.span_numbers[span_id])
            .product::<u32>();
        spams_sum += span_product;
    }

    Ok(spams_sum.to_string())
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
        let expected = "4361";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = 467835;
        let actual = part2(file_data)?;
        assert_eq!(actual, expected.to_string());
        Ok(())
    }
}
