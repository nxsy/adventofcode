use std::collections::BTreeMap;

use anyhow::{Context, Result};
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug)]
struct Game {
    input: Vec<char>,
    contiguous_damaged: Vec<u32>,
}

fn parse_line(input: &str) -> Result<Game> {
    // #.#.### 1,1,3
    let (input, damage) = input.split_once(' ').context("malformed input")?;
    let damage = damage
        .split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Game {
        input: input.chars().collect(),
        contiguous_damaged: damage,
    })
}

fn score_input(input: &str, target: &[u32]) -> bool {
    let mut last_run = 0;
    let mut runs = Vec::new();
    for ch in input.chars() {
        match ch {
            '#' => {
                last_run += 1;
            }
            '.' => {
                if last_run > 0 {
                    runs.push(last_run);
                    last_run = 0;
                }
            }
            c => panic!("invalid character {} in input {}", c, input),
        }
    }
    if last_run > 0 {
        runs.push(last_run);
    }
    runs == target
}

fn partial_match_input(input: &str, target: &[u32]) -> bool {
    let mut last_run = 0;
    let mut runs = Vec::new();
    let num_target_runs = target.len();
    let mut run_number = 0;

    for ch in input.chars() {
        match ch {
            '#' => {
                last_run += 1;
                if run_number == num_target_runs {
                    return false;
                }
                if last_run > target[run_number] {
                    return false;
                }
            }
            '.' => {
                if last_run > 0 {
                    if last_run != target[run_number] {
                        return false;
                    }
                    if run_number == num_target_runs {
                        return false;
                    }
                    run_number += 1;
                    runs.push(last_run);
                    last_run = 0;
                }
            }
            '?' => break,
            c => panic!("invalid character {} in input {}", c, input),
        }
    }
    true
}

fn solve_bf(input: &str, target: &[u32]) -> u32 {
    match input.find('?') {
        Some(p) => {
            let mut score = 0;
            for ch in &['#', '.'] {
                let mut input = input.to_string();
                input.replace_range(p..=p, &ch.to_string());
                let possible = partial_match_input(&input, target);
                if possible {
                    score += solve_bf(&input, target);
                }
            }
            score
        }
        None => {
            if score_input(input, target) {
                1
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct State {
    position: u32,
    block_index: u32,
    current_block_value: u32,
}

fn solve(input: &[char], target: &[u32], state: State, memo: &mut BTreeMap<State, u64>) -> u64 {
    if let Some(&score) = memo.get(&state) {
        return score;
    }

    if state.position == input.len() as u32 {
        if state.block_index == target.len() as u32 && state.current_block_value == 0 {
            // Ended with a '.' such that we started a new block but it has nothing in it.
            return 1;
        }
        if state.block_index == target.len() as u32 - 1
            && state.current_block_value == target[state.block_index as usize]
        {
            // Ended with a '#' such that we completed the exact right size of block at the end of the input.
            return 1;
        }
        return 0;
    }

    let mut score = 0;

    let chars = match input[state.position as usize] {
        '?' => vec!['#', '.'],
        c @ '#' | c @ '.' => vec![c],
        c => panic!("invalid character {} in input {:?}", c, input),
    };

    for c in chars {
        if c == '.' && state.current_block_value == 0 {
            // No existing block to complete before moving on
            let new_state = State {
                position: state.position + 1,
                block_index: state.block_index,
                current_block_value: state.current_block_value,
            };
            score += solve(input, target, new_state, memo);
        } else if c == '.' {
            if state.block_index >= target.len() as u32 {
                // Already completed all the blocks, so we can't add another
                continue;
            }
            if state.current_block_value != target[state.block_index as usize] {
                // The block we completed doesn't match the block we're supposed to complete
                continue;
            }
            let new_state = State {
                position: state.position + 1,
                block_index: state.block_index + 1,
                current_block_value: 0,
            };
            score += solve(input, target, new_state, memo);
        } else if c == '#' {
            if state.block_index >= target.len() as u32 {
                // Already completed all the blocks, so...
                continue;
            }

            if state.current_block_value >= target[state.block_index as usize] {
                // We've already completed this block, so...
                continue;
            }

            let new_state = State {
                position: state.position + 1,
                block_index: state.block_index,
                current_block_value: state.current_block_value + 1,
            };
            score += solve(input, target, new_state, memo);
        }
    }
    memo.insert(state, score);
    score
}

fn part1(input: &str) -> Result<String> {
    let mut total = 0;
    for line in input.lines() {
        let game = parse_line(line)?;
        // println!("{:?}", game);

        let s = solve_bf(
            &game.input.iter().collect::<String>(),
            &game.contiguous_damaged,
        );
        // let mut memo = BTreeMap::new();
        // let s = solve_dp(
        //     &game.input,
        //     &game.contiguous_damaged,
        //     State::default(),
        //     &mut memo,
        // );
        total += s;
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String> {
    let mut total = 0;
    let mut total_memo_size = 0;
    for line in input.lines() {
        let game = parse_line(line)?;

        let input = std::iter::repeat(game.input.iter().collect::<String>())
            .take(5)
            .join("?");
        let contiguous_damaged = std::iter::repeat(game.contiguous_damaged.clone())
            .take(5)
            .flatten()
            .collect::<Vec<_>>();

        let mut memo = BTreeMap::new();
        let s = solve(
            &input.chars().collect::<Vec<_>>(),
            &contiguous_damaged,
            State::default(),
            &mut memo,
        );

        total_memo_size += memo.len();
        total += s;
    }
    println!("total memo size: {}", total_memo_size);
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
        for (input, expected) in [
            ("???.### 1,1,3", "1"),
            (".??..??...?##. 1,1,3", "4"),
            ("?#?#?#?#?#?#?#? 1,3,1,6", "1"),
            ("????.#...#... 4,1,1", "1"),
            ("????.######..#####. 1,6,5", "4"),
            ("?###???????? 3,2,1", "10"),
        ] {
            let actual = part1(input)?;

            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        for (input, expected) in [
            ("???.### 1,1,3", "1"),
            (".??..??...?##. 1,1,3", "16384"),
            ("?#?#?#?#?#?#?#? 1,3,1,6", "1"),
            ("????.#...#... 4,1,1", "16"),
            ("????.######..#####. 1,6,5", "2500"),
            ("?###???????? 3,2,1", "506250"),
        ] {
            let actual = part2(input)?;

            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_score_input() -> Result<()> {
        for (input, target) in [
            ("#.#.###", vec![1, 1, 3]),
            (".#...#....###.", vec![1, 1, 3]),
            (".#.###.#.######", vec![1, 3, 1, 6]),
            ("####.#...#...", vec![4, 1, 1]),
            ("#....######..#####.", vec![1, 6, 5]),
            (".###.##....#", vec![3, 2, 1]),
        ] {
            let actual = score_input(input, &target);
            assert_eq!(actual, true);
        }
        Ok(())
    }

    #[test]
    fn test_partial_match_input() -> Result<()> {
        for (input, target, expected) in [
            ("#.#.###", vec![1, 1, 3], true),
            ("#.#.#??", vec![1, 1, 3], true),
            ("##?.#??", vec![1, 1, 3], false),
            (".#...#....###.", vec![1, 1, 3], true),
            (".#.###.#.######", vec![1, 3, 1, 6], true),
            ("####.#...#...", vec![4, 1, 1], true),
            ("#....######..#####.", vec![1, 6, 5], true),
            (".###.##....#", vec![3, 2, 1], true),
        ] {
            let actual = partial_match_input(input, &target);
            assert_eq!(actual, expected, "input: {}, target: {:?}", input, &target);
        }
        Ok(())
    }
}
