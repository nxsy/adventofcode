use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Pos3 {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Pos3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos: [i32; 3] = s
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Pos3 {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Block(Pos3, Pos3);

impl Block {
    fn overlap_xy(&self, other: &Self) -> bool {
        self.0.x <= other.1.x
            && self.1.x >= other.0.x
            && self.0.y <= other.1.y
            && self.1.y >= other.0.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum SupportedBy {
    Ground,
    Block(usize),
}

fn parse(input: &str) -> Result<Vec<Block>> {
    input
        .lines()
        .map(|line| -> Result<Block> {
            let (pos1, pos2) = line.split_once('~').context("failed to split on ~")?;
            Ok(Block(pos1.parse()?, pos2.parse()?))
        })
        .collect()
}

fn part1(input: &str) -> Result<String> {
    let blocks = parse(input)?;

    let mut blocks: Vec<(usize, Block)> = blocks.into_iter().enumerate().collect();

    let (_, supported_by) = flatten(&mut blocks);

    let mut total = 0;
    for (i, _) in &blocks {
        let i_set = BTreeSet::from([SupportedBy::Block(*i)]);
        if !supported_by.values().any(|s| s == &i_set) {
            total += 1;
        }
    }

    Ok(total.to_string())
}

fn flatten(blocks: &mut [(usize, Block)]) -> (usize, BTreeMap<usize, BTreeSet<SupportedBy>>) {
    let max_z = blocks.iter().map(|(_, b)| b.0.z).max().unwrap();

    let mut num_fell = 0;

    let mut settled = Vec::new();
    for z in 1..max_z + 1 {
        let blocks_at_level = blocks.iter_mut().filter(|(_, b)| b.0.z == z);
        if z == 1 {
            for (i, b) in blocks_at_level {
                settled.push((*i, *b, BTreeSet::from([SupportedBy::Ground])));
            }
            continue;
        }
        for (i, b) in blocks.iter_mut().filter(|(_, b)| b.0.z == z) {
            let highest_overlaps = settled
                .iter()
                .filter(|(_, b2, _)| b.overlap_xy(b2))
                .map(|(i, b2, _)| (b2.1.z, *i, *b2))
                .sorted()
                .group_by(|(z, ..)| *z)
                .into_iter()
                .max_by_key(|(z, ..)| *z)
                .map(|(_, g)| g.into_iter().collect_vec());
            let (distance_to_fall, supported_by) = if let Some(highest_overlaps) = highest_overlaps
            {
                (
                    b.0.z - highest_overlaps[0].0 - 1,
                    highest_overlaps
                        .into_iter()
                        .map(|(_, i, _)| SupportedBy::Block(i))
                        .collect(),
                )
            } else {
                (z - 1, BTreeSet::from([SupportedBy::Ground]))
            };
            if distance_to_fall > 0 {
                b.0.z -= distance_to_fall;
                b.1.z -= distance_to_fall;
                num_fell += 1;
            }

            settled.push((*i, *b, supported_by));
        }
    }

    let supported_by = settled
        .into_iter()
        .map(|(i, _, supported_by)| (i, supported_by))
        .collect::<BTreeMap<_, _>>();

    (num_fell, supported_by)
}

fn part2(input: &str) -> Result<String> {
    let blocks = parse(input)?;

    let mut blocks: Vec<(usize, Block)> = blocks.into_iter().enumerate().collect();

    let (_, supported_by) = flatten(&mut blocks);

    let mut total = 0;
    for &b in &blocks {
        let mut fallen = BTreeSet::from([SupportedBy::Block(b.0)]);

        loop {
            let mut local_total = 0;
            for (j, supported_by) in &supported_by {
                if supported_by.is_subset(&fallen) && fallen.insert(SupportedBy::Block(*j)) {
                    local_total += 1;
                }
            }
            total += local_total;
            if local_total == 0 {
                break;
            }
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
        let expected = "5";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "7";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
