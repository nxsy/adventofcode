use std::collections::{HashMap, HashSet};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy)]
enum Dir {
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

struct Game<'a> {
    pattern: Vec<Dir>,
    nodes: HashMap<&'a str, Node<'a>>,
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    // AAA = (BBB, CCC)
    let (input, (name, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)?;
    Ok((input, Node { name, left, right }))
}

fn parse_data(input: &str) -> IResult<&str, Game> {
    let (input, pattern) = many1(one_of("LR"))(input)?;
    let pattern = pattern
        .iter()
        .map(|c| match c {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let (input, _) = tuple((line_ending, line_ending))(input)?;
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;

    Ok((
        input,
        Game {
            pattern,
            nodes: nodes.into_iter().map(|n| (n.name, n)).collect(),
        },
    ))
}

fn part1(input: &'static str) -> Result<String> {
    let (_, data) = parse_data(input)?;
    let mut pos = "AAA";
    let mut steps = 0;
    for dir in data.pattern.iter().cycle() {
        steps += 1;
        let node = data.nodes.get(pos).unwrap();
        pos = match dir {
            Dir::L => node.left,
            Dir::R => node.right,
        };
        if pos == "ZZZ" {
            break;
        }
    }
    Ok(steps.to_string())
}

#[derive(Debug, Clone)]
enum SearchResult {
    None,
    FoundOne(usize),
    FoundTwo(usize, usize),
}

fn part2(input: &'static str) -> Result<String> {
    let (_, data) = parse_data(input)?;
    let mut positions: Vec<_> = data
        .nodes
        .keys()
        .copied()
        .filter(|n| n.ends_with('A'))
        .collect();
    let mut steps = 0;

    let mut positions_found_at_steps = vec![SearchResult::None; positions.len()];
    let mut steps_complete = HashSet::new();

    for dir in data.pattern.iter().cycle() {
        steps += 1;
        for (i, pos) in positions.iter_mut().enumerate() {
            let node = data.nodes.get(pos).unwrap();
            *pos = match dir {
                Dir::L => node.left,
                Dir::R => node.right,
            };
            if pos.ends_with('Z') {
                positions_found_at_steps[i] = match &positions_found_at_steps[i] {
                    SearchResult::None => SearchResult::FoundOne(steps),
                    SearchResult::FoundOne(original_step) => {
                        steps_complete.insert(steps);
                        SearchResult::FoundTwo(*original_step, steps)
                    }
                    z => z.clone(),
                };
            }
        }
        if steps_complete.len() == positions.len() {
            break;
        }
    }

    let lcm = positions_found_at_steps.iter().fold(1, |acc, n| {
        if let SearchResult::FoundTwo(a, b) = n {
            num::integer::lcm(acc, *b - *a)
        } else {
            acc
        }
    });

    Ok(lcm.to_string())
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
        let expected = "2";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);

        let file_data = include_str!("example_input2.txt");
        let expected = "6";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input3.txt");
        let expected = "6";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
