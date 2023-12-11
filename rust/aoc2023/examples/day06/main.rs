use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::space1,
    character::complete::{line_ending, u32 as nom_u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, times) = preceded(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, nom_u32),
    )(input)?;
    let (input, distances) = preceded(
        tuple((line_ending, tag("Distance:"), space1)),
        separated_list1(space1, nom_u32),
    )(input)?;
    Ok((input, times.into_iter().zip(distances).collect()))
}

fn part1(input: &'static str) -> Result<String> {
    let (_, data) = parse(input).unwrap();

    let mut product = 1;
    for (time, record_distance) in data {
        let mut wins = 0;
        for time_pressed in 1..=time {
            let speed = time_pressed;
            let moving = time - time_pressed;
            let distance_moved = speed * moving;
            if distance_moved > record_distance {
                wins += 1;
            }
        }
        product *= wins;
    }
    Ok(product.to_string())
}

fn part2(input: &str) -> Result<String> {
    let (_, data) = parse(input).unwrap();

    let time: String = data.iter().map(|(t, _)| t.to_string()).collect();
    let time: u64 = time.parse()?;
    let distance: String = data.iter().map(|(_, d)| d.to_string()).collect();
    let record_distance: u64 = distance.parse()?;

    let mut wins = 0;
    for time_pressed in 1..=time {
        let speed = time_pressed;
        let moving = time - time_pressed;
        let distance_moved = speed * moving;
        if distance_moved > record_distance {
            wins += 1;
        }
    }
    Ok(wins.to_string())
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
        let expected = "288";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "71503";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
