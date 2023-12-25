use anyhow::Result;

use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

use crate::{Hailstone, ParsedInput, Pos};

fn parse_pos(input: &str) -> IResult<&str, Pos> {
    let (input, (x, y, z)) = tuple((
        terminated(nom_i64, tuple((tag(","), space1))),
        terminated(nom_i64, tuple((tag(","), space1))),
        nom_i64,
    ))(input)?;
    Ok((
        input,
        Pos {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        },
    ))
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (input, (pos, dir)) =
        separated_pair(parse_pos, delimited(space1, tag("@"), space1), parse_pos)(input)?;
    Ok((input, Hailstone { pos, dir }))
}

pub(crate) fn parse(input: &'static str) -> Result<ParsedInput> {
    let (_, hailstones) = all_consuming(separated_list1(line_ending, parse_hailstone))(input)?;
    Ok(ParsedInput { hailstones })
}
