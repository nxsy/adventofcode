use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, i64 as nom_i64, line_ending},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult, Parser,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Condition {
    attribute: &'static str,
    operator: Operator,
    value: i64,
    outcome: Outcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Accept,
    Reject,
    Workflow(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    conditions: Vec<Condition>,
    default_outcome: Outcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operator {
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    workflows: BTreeMap<&'static str, Workflow>,
    parts: Vec<Part>,
}

fn parse_outcome(input: &'static str) -> IResult<&str, Outcome> {
    let (input, outcome) = alt((
        tag("A").map(|_| Outcome::Accept),
        tag("R").map(|_| Outcome::Reject),
        alpha1.map(Outcome::Workflow),
    ))(input)?;
    Ok((input, outcome))
}

fn parse_workflow(input: &'static str) -> IResult<&str, (&str, Workflow)> {
    let (input, name) = alpha1(input)?;
    let (input, (conditions, default_outcome)) = delimited(
        tag("{"),
        separated_pair(
            separated_list1(
                tag(","),
                tuple((
                    alpha1,
                    alt((
                        char('>').map(|_| Operator::GreaterThan),
                        char('<').map(|_| Operator::LessThan),
                    )),
                    nom_i64,
                    preceded(char(':'), parse_outcome),
                ))
                .map(|(attribute, operator, value, outcome)| Condition {
                    attribute,
                    operator,
                    value,
                    outcome,
                }),
            ),
            tag(","),
            parse_outcome,
        ),
        tag("}"),
    )(input)?;
    Ok((
        input,
        (
            name,
            Workflow {
                conditions,
                default_outcome,
            },
        ),
    ))
}

fn parse_workflows(input: &'static str) -> IResult<&str, Vec<(&str, Workflow)>> {
    let (input, workflows) = separated_list1(line_ending, parse_workflow)(input)?;
    Ok((input, workflows))
}

fn parse_parts(input: &'static str) -> IResult<&str, Vec<Part>> {
    let (input, parts) = separated_list1(
        line_ending,
        tuple((
            preceded(tag("{x="), nom_i64),
            preceded(tag(",m="), nom_i64),
            preceded(tag(",a="), nom_i64),
            preceded(tag(",s="), nom_i64),
            tag("}"),
        ))
        .map(|(x, m, a, s, _)| Part { x, m, a, s }),
    )(input)?;
    Ok((input, parts))
}

fn parse(input: &'static str) -> IResult<&str, Game> {
    let (input, (workflows, parts)) = separated_pair(
        parse_workflows,
        tuple((line_ending, line_ending)),
        parse_parts,
    )(input)?;
    let workflows = workflows.into_iter().collect();
    Ok((input, Game { workflows, parts }))
}

fn part1(input: &'static str) -> Result<String> {
    let (_, game) = parse(input)?;

    let mut total = 0;
    for part in &game.parts {
        let game = &game;
        let mut result = Outcome::Workflow("in");

        while let Outcome::Workflow(name) = result {
            let workflow = game.workflows.get(name).unwrap();

            let mut wf_result = None;
            for condition in workflow.conditions.iter() {
                let value = match condition.attribute {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => unreachable!(),
                };
                let outcome = match condition.operator {
                    Operator::GreaterThan => value > condition.value,
                    Operator::LessThan => value < condition.value,
                };
                if outcome {
                    wf_result = Some(condition.outcome);
                    break;
                }
            }
            result = wf_result.unwrap_or(workflow.default_outcome);
        }
        if let Outcome::Accept = result {
            total += part.x + part.m + part.a + part.s;
        }
    }
    Ok(total.to_string())
}

fn part2(input: &'static str) -> Result<String> {
    let (_, game) = parse(input)?;

    let mut consider_ranges = vec![("in", [(1, 4001), (1, 4001), (1, 4001), (1, 4001)])];

    let mut total = 0;
    while let Some((workflow_name, mut ranges)) = consider_ranges.pop() {
        let workflow = game.workflows.get(workflow_name).unwrap();

        let mut outcomes = Vec::new();

        for condition in workflow.conditions.iter() {
            let range_idx = match condition.attribute {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => unreachable!(),
            };

            let value_to_split = match condition.operator {
                Operator::GreaterThan => condition.value + 1,
                Operator::LessThan => condition.value,
            };

            let mut ranges1 = ranges;
            ranges1[range_idx] = (ranges1[range_idx].0, value_to_split);
            let mut ranges2 = ranges;
            ranges2[range_idx] = (value_to_split, ranges2[range_idx].1);

            let (matched_ranges, unmatched_ranges) = match condition.operator {
                Operator::GreaterThan => (ranges2, ranges1),
                Operator::LessThan => (ranges1, ranges2),
            };

            outcomes.push((matched_ranges, condition.outcome));

            ranges = unmatched_ranges;
        }

        outcomes.push((ranges, workflow.default_outcome));

        for (ranges, outcome) in outcomes {
            match outcome {
                Outcome::Accept => {
                    total += (ranges[0].1 - ranges[0].0)
                        * (ranges[1].1 - ranges[1].0)
                        * (ranges[2].1 - ranges[2].0)
                        * (ranges[3].1 - ranges[3].0)
                }
                Outcome::Workflow(name) => {
                    consider_ranges.push((name, ranges));
                }
                Outcome::Reject => {}
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
        let expected = "19114";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "167409079868000";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
