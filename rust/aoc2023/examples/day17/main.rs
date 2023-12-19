use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone)]
struct Grid {
    data: BTreeMap<(i32, i32), i64>,
    extents: (i32, i32, i32, i32),
}

fn parse(input: &str) -> Result<Grid> {
    let mut data = BTreeMap::new();
    let mut extents = (0, 0, 0, 0);

    for (y, line) in input.trim().lines().enumerate() {
        (extents.2, extents.3) = (extents.2.min(y as i32), extents.3.max(y as i32));

        for (x, ch) in line.chars().enumerate() {
            (extents.0, extents.1) = (extents.0.min(x as i32), extents.1.max(x as i32));

            let value = ch.to_digit(10).ok_or_else(|| anyhow!("invalid digit"))? as i64;
            data.insert((x as i32, y as i32), value);
        }
    }
    Ok(Grid { data, extents })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct PosWithCost {
    cost: Reverse<i64>,
    pos: (i32, i32),
    recent_history: Option<(Direction, u8)>,
}

type Pos = (i32, i32);
type RecentHistory = Option<(Direction, u8)>;

fn solve(
    grid: &Grid,
    min_move_in_direction: u8,
    max_move_in_direction: u8,
) -> std::prelude::v1::Result<String, anyhow::Error> {
    let starting_pos = (0, 0);
    let ending_pos = (grid.extents.1, grid.extents.3);
    let mut stack = BinaryHeap::new();

    stack.push(PosWithCost {
        pos: starting_pos,
        recent_history: None,
        cost: Reverse(0),
    });

    let mut visited: BTreeMap<(Pos, RecentHistory), i64> = BTreeMap::new();

    while let Some(pos_with_cost) = stack.pop() {
        let PosWithCost {
            pos,
            recent_history,
            cost: Reverse(cost),
        } = pos_with_cost;

        if visited.contains_key(&(pos, recent_history)) {
            continue;
        }
        visited.insert((pos, recent_history), cost);

        let opposite_direction = recent_history.map(|(d, _)| d.opposite());

        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if Some(d) == opposite_direction {
                continue;
            }
            match recent_history {
                Some((prev_direction, num_steps))
                    if d != prev_direction && num_steps < min_move_in_direction =>
                {
                    continue;
                }
                _ => {}
            }
            let new_num_steps = match recent_history {
                Some((prev_direction, num_steps)) if d == prev_direction => num_steps + 1,
                _ => 1,
            };
            if new_num_steps > max_move_in_direction {
                continue;
            }
            let delta = d.delta();
            let neighbor = (pos.0 + delta.0, pos.1 + delta.1);
            let Some(value) = grid.data.get(&neighbor) else {
                continue;
            };
            if neighbor == ending_pos && new_num_steps < min_move_in_direction {
                continue;
            }
            stack.push(PosWithCost {
                pos: neighbor,
                recent_history: Some((d, new_num_steps)),
                cost: Reverse(cost + value),
            });
        }
    }

    visited
        .iter()
        .filter_map(|(k, v)| if k.0 == ending_pos { Some(v) } else { None })
        .min()
        .map(|v| v.to_string())
        .ok_or_else(|| anyhow!("no ending pos"))
}

fn part1(input: &str) -> Result<String> {
    let grid = parse(input)?;
    let min_move_in_direction = 0;
    let max_move_in_direction = 3;

    solve(&grid, min_move_in_direction, max_move_in_direction)
}

fn part2(input: &str) -> Result<String> {
    let grid = parse(input)?;
    let min_move_in_direction = 4;
    let max_move_in_direction = 10;

    solve(&grid, min_move_in_direction, max_move_in_direction)
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
        let expected = "102";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "94";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2_2() -> Result<()> {
        let file_data = include_str!("example_input2.txt");
        let expected = "71";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
