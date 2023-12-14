use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

fn parse(input: &str) -> Result<Vec<BTreeSet<(i64, i64)>>> {
    let mut result = Vec::new();
    // Ug.  Why no cross-platform way to split on double newlines?
    for lines in input.split("\r\n\r\n") {
        let mut set = BTreeSet::new();
        for (y, line) in lines.lines().enumerate() {
            for (x, ch) in line.char_indices() {
                if ch == '#' {
                    set.insert((x as i64 + 1, y as i64 + 1));
                }
            }
        }

        result.push(set);
    }
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Mirror {
    X(i64),
    Y(i64),
}

fn reflect(pos: (i64, i64), mirror: Mirror) -> (i64, i64) {
    match mirror {
        Mirror::X(x) => (x + (x - pos.0) + 1, pos.1),
        Mirror::Y(y) => (pos.0, y + (y - pos.1) + 1),
    }
}

fn part1(input: &str) -> Result<String> {
    let mut total = 0;
    for set in parse(input)? {
        let (min_x, max_x, min_y, max_y) = set.iter().fold(
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
            },
        );

        let mut r = None;

        for x in min_x..max_x {
            let mut is_mirror = true;
            let m = Mirror::X(x);
            for &pos in &set {
                let mirror_pos = reflect(pos, m);
                if mirror_pos.0 < min_x || mirror_pos.0 > max_x {
                    continue;
                }

                if !set.contains(&mirror_pos) {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                r = Some(m);
                break;
            }
        }

        if r.is_none() {
            for y in min_y..max_y {
                let mut is_mirror = true;
                let m = Mirror::Y(y);
                for &pos in &set {
                    let mirror_pos = reflect(pos, m);
                    if mirror_pos.1 < min_y || mirror_pos.1 > max_y {
                        continue;
                    }

                    if !set.contains(&mirror_pos) {
                        is_mirror = false;
                        break;
                    }
                }
                if is_mirror {
                    r = Some(m);
                    break;
                }
            }
        }

        match r {
            Some(Mirror::X(n)) => {
                total += n;
            }
            Some(Mirror::Y(n)) => {
                total += 100 * n;
            }
            None => {
                panic!("no mirror found");
            }
        }
    }
    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String> {
    let mut total = 0;
    for set in parse(input)? {
        let (min_x, max_x, min_y, max_y) = set.iter().fold(
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
            },
        );

        let mut options = BTreeMap::new();

        for x in min_x..max_x {
            let mut imperfections = Vec::new();
            let m = Mirror::X(x);
            for &pos in &set {
                let mirror_pos = reflect(pos, m);
                if mirror_pos.0 < min_x || mirror_pos.0 > max_x {
                    continue;
                }

                if !set.contains(&mirror_pos) {
                    imperfections.push(mirror_pos);
                }
            }
            options.insert(m, imperfections);
        }

        for y in min_y..max_y {
            let mut imperfections = Vec::new();
            let m = Mirror::Y(y);
            for &pos in &set {
                let mirror_pos = reflect(pos, m);
                if mirror_pos.1 < min_y || mirror_pos.1 > max_y {
                    continue;
                }

                if !set.contains(&mirror_pos) {
                    imperfections.push(mirror_pos);
                }
            }
            options.insert(m, imperfections);
        }

        let r = options
            .iter()
            .filter_map(|n| (n.1.len() == 1).then_some(*n.0))
            .exactly_one()
            .map_err(|_| anyhow!("More than one mirror found"))?;

        match r {
            Mirror::X(n) => {
                total += n;
            }
            Mirror::Y(n) => {
                total += 100 * n;
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
        let expected = "405";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_reflect() -> Result<()> {
        let cases = [
            ((4, 1), Mirror::X(5), (7, 1)),
            ((5, 2), Mirror::X(5), (6, 2)),
            ((3, 1), Mirror::X(5), (8, 1)),
            ((8, 1), Mirror::X(5), (3, 1)),
            ((3, 3), Mirror::Y(4), (3, 6)),
        ];
        for (pos, mirror, expected) in &cases {
            let actual = reflect(*pos, *mirror);
            assert_eq!(actual, *expected);
        }
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "400";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
