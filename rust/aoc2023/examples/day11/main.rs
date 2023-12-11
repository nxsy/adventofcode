use std::collections::BTreeSet;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(input: &'static str) -> Result<String> {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, ch)| (ch == '#').then_some((x as i64, y as i64)))
        })
        .collect::<Vec<_>>();

    let column_with_content = galaxies.iter().map(|(x, _)| *x).collect::<BTreeSet<_>>();
    let row_with_content = galaxies.iter().map(|(_, y)| *y).collect::<BTreeSet<_>>();

    let mut galaxies = galaxies.clone();

    for (x, y) in galaxies.iter_mut() {
        for tx in 0..*x {
            if !column_with_content.contains(&tx) {
                *x += 1;
            }
        }
        for ty in 0..*y {
            if !row_with_content.contains(&ty) {
                *y += 1;
            }
        }
    }

    let mut total_distance = 0;
    for (a, galaxy_a) in galaxies.iter().enumerate() {
        for (b, galaxy_b) in galaxies.iter().enumerate() {
            if a >= b {
                continue;
            }
            total_distance += distance(*galaxy_a, *galaxy_b);
        }
    }

    Ok(total_distance.to_string())
}

fn part2(input: &str) -> Result<String> {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, ch)| (ch == '#').then_some((x as i64, y as i64)))
        })
        .collect::<Vec<_>>();

    let column_with_content = galaxies.iter().map(|(x, _)| *x).collect::<BTreeSet<_>>();
    let row_with_content = galaxies.iter().map(|(_, y)| *y).collect::<BTreeSet<_>>();

    let mut galaxies = galaxies.clone();

    // let resize = 9;
    // let resize = 99;
    let resize = 999999;
    for (x, y) in galaxies.iter_mut() {
        for tx in 0..*x {
            if !column_with_content.contains(&tx) {
                *x += resize;
            }
        }
        for ty in 0..*y {
            if !row_with_content.contains(&ty) {
                *y += resize;
            }
        }
    }

    let mut total_distance = 0;
    for (a, galaxy_a) in galaxies.iter().enumerate() {
        for (b, galaxy_b) in galaxies.iter().enumerate() {
            if a >= b {
                continue;
            }
            total_distance += distance(*galaxy_a, *galaxy_b);
        }
    }

    Ok(total_distance.to_string())
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
        let expected = "374";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        // let expected = "1030";
        let expected = "8410";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
