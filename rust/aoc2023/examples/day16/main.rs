use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy)]
enum MirrorType {
    MirrorPositive,
    MirrorNegative,
    SplitterUpDown,
    SplitterLeftRight,
}

impl FromStr for MirrorType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/" => Ok(MirrorType::MirrorPositive),
            "\\" => Ok(MirrorType::MirrorNegative),
            "|" => Ok(MirrorType::SplitterUpDown),
            "-" => Ok(MirrorType::SplitterLeftRight),
            _ => bail!("invalid mirror type: {}", s),
        }
    }
}

impl MirrorType {
    fn as_char(&self) -> char {
        match self {
            MirrorType::MirrorPositive => '/',
            MirrorType::MirrorNegative => '\\',
            MirrorType::SplitterUpDown => '|',
            MirrorType::SplitterLeftRight => '-',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
struct Game {
    map: BTreeMap<(i32, i32), MirrorType>,
    extents: (i32, i32, i32, i32),
}

impl Game {
    #[allow(dead_code)]
    fn as_string(&self) -> String {
        let mut output = String::new();
        for y in self.extents.2..=self.extents.3 {
            for x in self.extents.0..=self.extents.1 {
                let pos = (x, y);
                let ch = match self.map.get(&pos) {
                    Some(mirror) => mirror.as_char(),
                    None => '.',
                };
                output.push(ch);
            }
            output.push('\n');
        }
        output
    }
}

fn apply_mirror(mirror: &MirrorType, dir: Direction) -> Vec<Direction> {
    match mirror {
        MirrorType::MirrorPositive => match dir {
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
        },
        MirrorType::MirrorNegative => match dir {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
        },
        MirrorType::SplitterUpDown => match dir {
            Direction::Up | Direction::Down => vec![dir],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        },
        MirrorType::SplitterLeftRight => match dir {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![dir],
        },
    }
}

fn find_energized(game: &Game, starting_position: (i32, i32, Direction)) -> BTreeSet<(i32, i32)> {
    let mut seen = BTreeSet::new();
    let mut energized = BTreeSet::new();
    let mut to_do = vec![starting_position];

    while let Some((x, y, dir)) = to_do.pop() {
        let pos = (x, y);
        if !seen.insert((pos, dir)) {
            continue;
        }
        energized.insert(pos);

        let new_dirs = if let Some(mirror) = game.map.get(&pos) {
            apply_mirror(mirror, dir)
        } else {
            vec![dir]
        };

        for d in new_dirs {
            let (dx, dy) = d.delta();
            let new_pos = (x + dx, y + dy);
            if new_pos.0 < game.extents.0
                || new_pos.0 > game.extents.1
                || new_pos.1 < game.extents.2
                || new_pos.1 > game.extents.3
            {
                continue;
            }
            to_do.push((new_pos.0, new_pos.1, d));
        }
    }
    energized
}

fn parse(input: &'static str) -> Result<Game> {
    let mut map = BTreeMap::new();
    let mut extents = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for (y, line) in input.lines().enumerate() {
        (extents.2, extents.3) = (extents.2.min(y as i32), extents.3.max(y as i32));
        for (x, ch) in line.chars().enumerate() {
            (extents.0, extents.1) = (extents.0.min(x as i32), extents.1.max(x as i32));
            let pos = (x as i32, y as i32);
            if let Ok(mirror) = ch.to_string().parse::<MirrorType>() {
                map.insert(pos, mirror);
            }
        }
    }
    Ok(Game { map, extents })
}

fn part1(input: &'static str) -> Result<String> {
    let game = parse(input)?;

    let starting_position = (0, 0, Direction::Right);
    let energized = find_energized(&game, starting_position);

    Ok(energized.len().to_string())
}

fn part2(input: &'static str) -> Result<String> {
    let game = parse(input)?;

    let mut positions_to_test = Vec::new();
    for y in game.extents.2..=game.extents.3 {
        positions_to_test.push((game.extents.0, y, Direction::Right));
        positions_to_test.push((game.extents.1, y, Direction::Left));
    }
    for x in game.extents.0..=game.extents.1 {
        positions_to_test.push((x, game.extents.2, Direction::Down));
        positions_to_test.push((x, game.extents.3, Direction::Up));
    }

    let mut positions_to_energized = BTreeMap::new();
    for starting_position in positions_to_test {
        let energized = find_energized(&game, starting_position);
        positions_to_energized.insert(starting_position, energized);
    }

    let total = positions_to_energized
        .values()
        .map(|energized| energized.len())
        .max()
        .unwrap();
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
        let expected = "46";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "51";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
