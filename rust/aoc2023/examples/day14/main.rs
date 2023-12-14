use std::collections::{BTreeSet, HashMap};

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug)]
struct Game {
    rolling_rocks: BTreeSet<(i64, i64)>,
    cube_rocks: BTreeSet<(i64, i64)>,

    extents: (i64, i64, i64, i64),
}

impl Game {
    fn output(&self) -> String {
        let mut output = String::new();
        for y in self.extents.2..=self.extents.3 {
            for x in self.extents.0..=self.extents.1 {
                output.push(if self.cube_rocks.contains(&(x, y)) {
                    '#'
                } else if self.rolling_rocks.contains(&(x, y)) {
                    'O'
                } else {
                    '.'
                });
            }
            if y != self.extents.3 {
                output.push('\n');
            }
        }
        output
    }
}

fn parse(input: &str) -> Result<Game> {
    let mut rolling_rocks = BTreeSet::new();
    let mut cube_rocks = BTreeSet::new();

    let (max_x, max_y) = input
        .lines()
        .enumerate()
        .fold((0, 0), |(max_x, max_y), (y, line)| {
            (max_x.max(line.len() - 1), max_y.max(y))
        });

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    cube_rocks.insert((x as i64, y as i64));
                }
                'O' => {
                    rolling_rocks.insert((x as i64, y as i64));
                }
                _ => bail!("invalid character"),
            }
        }
    }

    let game = Game {
        rolling_rocks,
        cube_rocks,
        extents: (0, max_x as i64, 0, max_y as i64),
    };
    Ok(game)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    North,
    West,
    South,
    East,
}

impl Dir {
    fn delta(&self) -> (i64, i64) {
        match self {
            Dir::North => (0, -1),
            Dir::West => (-1, 0),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
        }
    }

    fn at_border(&self, x: i64, y: i64, max_x: i64, max_y: i64) -> bool {
        match self {
            Dir::North => y == 0,
            Dir::West => x == 0,
            Dir::South => y == max_y,
            Dir::East => x == max_x,
        }
    }
}

fn can_go(
    x: i64,
    y: i64,
    dir: Dir,
    max_x: i64,
    max_y: i64,
    cube_rocks: &BTreeSet<(i64, i64)>,
    rolling_rocks: &BTreeSet<(i64, i64)>,
) -> bool {
    if dir.at_border(x, y, max_x, max_y) {
        return false;
    }
    let (dx, dy) = dir.delta();

    if cube_rocks.contains(&(x + dx, y + dy)) {
        return false;
    }
    if rolling_rocks.contains(&(x + dx, y + dy)) {
        return false;
    }
    true
}

fn tilt(game: Game, dir: Dir) -> Game {
    let mut new_rolling_rocks = BTreeSet::new();

    let mut rolling_rocks_in_order = game.rolling_rocks.iter().collect::<Vec<_>>();
    match dir {
        Dir::North => rolling_rocks_in_order.sort_by_key(|(x, y)| (y, x)),
        Dir::West => rolling_rocks_in_order.sort_by_key(|(x, y)| (x, y)),
        Dir::South => rolling_rocks_in_order.sort_by_key(|(x, y)| (-y, x)),
        Dir::East => rolling_rocks_in_order.sort_by_key(|(x, y)| (-x, y)),
    }

    for &(mut x, mut y) in rolling_rocks_in_order.iter() {
        while can_go(
            x,
            y,
            dir,
            game.extents.1,
            game.extents.3,
            &game.cube_rocks,
            &new_rolling_rocks,
        ) {
            x += dir.delta().0;
            y += dir.delta().1;
        }
        new_rolling_rocks.insert((x, y));
    }
    Game {
        rolling_rocks: new_rolling_rocks,
        ..game
    }
}

fn part1(input: &str) -> Result<String> {
    let game = parse(input)?;
    let game = tilt(game, Dir::North);

    let total = game
        .rolling_rocks
        .iter()
        .map(|(_, y)| game.extents.3 + 1 - y)
        .sum::<i64>();

    Ok(total.to_string())
}

fn part2(input: &str) -> Result<String> {
    let game = parse(input)?;

    let mut seen: HashMap<String, (i64, Vec<i32>)> = HashMap::new();

    let mut game = game;
    let mut i = 0;
    let (cycle_period, cycle_start) = loop {
        for d in &[Dir::North, Dir::West, Dir::South, Dir::East] {
            game = tilt(game, *d);
        }
        let output = game.output();
        let total = game
            .rolling_rocks
            .iter()
            .map(|(_, y)| game.extents.3 + 1 - y)
            .sum::<i64>();
        let e = seen.entry(output.clone()).or_default();
        e.0 = total;
        e.1.push(i);
        if e.1.len() == 3 {
            break (e.1[1] - e.1[0], e.1[1]);
        }
        i += 1;
    };

    let target = 1_000_000_000 - cycle_start;
    let target = cycle_start + (target % cycle_period) - 1;

    let total = seen
        .iter()
        .find(|(_, v)| v.1.contains(&target))
        .unwrap()
        .1
         .0;

    // dbg!(game);
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
        let expected = "136";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "64";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_north() -> Result<()> {
        let game = parse(include_str!("example_input.txt"))?;
        let game = tilt(game, Dir::North);

        let expected = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

        assert_eq!(game.output(), expected);

        // dbg!(game);
        Ok(())
    }

    #[test]
    fn test_cycles() -> Result<()> {
        let mut game = parse(include_str!("example_input.txt"))?;

        let expected = [
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        ];

        for (i, expected) in expected.iter().enumerate() {
            for dir in &[Dir::North, Dir::West, Dir::South, Dir::East] {
                game = tilt(game, *dir);
            }
            assert_eq!(&game.output(), expected);
        }

        // dbg!(game);
        Ok(())
    }
}
