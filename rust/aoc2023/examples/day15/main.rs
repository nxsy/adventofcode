use std::collections::BTreeMap;

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

fn hash(piece: &str) -> Result<u64> {
    let mut piece_total: u64 = 0;
    for ch in piece.chars() {
        let ascii_code: u8 = ch.try_into()?;
        piece_total += ascii_code as u64;
        piece_total *= 17;
        piece_total %= 256;
    }
    Ok(piece_total)
}

fn part1(input: &str) -> Result<String> {
    let input_pieces = input.trim().split(',').collect::<Vec<_>>();
    let mut total = 0;

    for piece in input_pieces {
        let piece_total = hash(piece)?;

        total += piece_total;
    }
    Ok(total.to_string())
}

#[derive(Debug, Clone, Copy)]
struct BoxEntry {
    label: &'static str,
    value: u64,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    AddToBox(BoxEntry),
    Remove(&'static str),
}

impl TryFrom<&'static str> for Op {
    type Error = anyhow::Error;

    fn try_from(piece: &'static str) -> Result<Self, Self::Error> {
        if let Some(piece) = piece.strip_suffix('-') {
            Ok(Op::Remove(piece))
        } else {
            let Some((label, value)) = piece.split_once('=') else {
                bail!("invalid input: {}", piece);
            };
            let value = value.parse::<u64>()?;
            Ok(Op::AddToBox(BoxEntry { label, value }))
        }
    }
}

fn part2(input: &'static str) -> Result<String> {
    let input_pieces = input.trim().split(',').collect::<Vec<_>>();

    let ops = input_pieces
        .iter()
        .map(|&piece| -> Result<Op> { Op::try_from(piece) })
        .collect::<Result<Vec<_>>>()?;

    let mut boxes: BTreeMap<u64, Vec<BoxEntry>> = BTreeMap::new();
    for op in ops {
        match op {
            Op::AddToBox(box_entry) => {
                let hash = hash(box_entry.label)?;
                let b = boxes.entry(hash).or_default();
                let mut found = false;
                for existing_box_entry in b.iter_mut() {
                    {
                        if existing_box_entry.label == box_entry.label {
                            existing_box_entry.value = box_entry.value;
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    b.push(box_entry);
                }
            }
            Op::Remove(label_to_remove) => {
                for (_, b) in boxes.iter_mut() {
                    *b = b
                        .iter()
                        .filter(|box_entry| box_entry.label != label_to_remove)
                        .copied()
                        .collect::<Vec<_>>();
                }
            }
        }
    }

    let mut total = 0;
    for (hash, b) in boxes.iter() {
        for (i, box_entry) in b.iter().enumerate() {
            total += (hash + 1) * (i as u64 + 1) * box_entry.value;
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
        let expected = "1320";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "145";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
