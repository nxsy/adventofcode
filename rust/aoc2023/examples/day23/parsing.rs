use std::collections::BTreeMap;

use anyhow::{bail, Result};

use crate::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn delta(&self) -> Pos {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug)]
pub(crate) struct Map {
    map: BTreeMap<Pos, Tile>,
    starting_position: Pos,
    target_position: Pos,
}

impl crate::Map for Map {
    fn traversable_tiles(&self) -> Vec<Pos> {
        let mut result = vec![];
        for (p, t) in &self.map {
            if !matches!(t, Tile::Forest) {
                result.push(*p);
            }
        }
        result
    }

    fn traversable_neighbors(&self, pos: Pos, follow_slopes: bool) -> Vec<Pos> {
        let mut neighbors = vec![];
        for d in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            let delta = d.delta();
            let neighbor = (pos.0 + delta.0, pos.1 + delta.1);
            match self.map.get(&neighbor) {
                Some(Tile::Path) => neighbors.push(neighbor),
                Some(Tile::Slope(slope)) if !follow_slopes || *slope == d => {
                    neighbors.push(neighbor)
                }
                _ => {}
            }
        }
        neighbors
    }

    fn get_starting_position(&self) -> Pos {
        self.starting_position
    }

    fn get_target_position(&self) -> Pos {
        self.target_position
    }
}

pub(crate) fn parse(input: &str) -> Result<Map> {
    let mut map = BTreeMap::new();
    let mut starting_position = (0, 0);
    let mut target_position = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '>' => Tile::Slope(Direction::Right),
                '<' => Tile::Slope(Direction::Left),
                '^' => Tile::Slope(Direction::Up),
                'v' => Tile::Slope(Direction::Down),
                _ => bail!("invalid character"),
            };
            if c == '.' {
                if y == 0 {
                    starting_position = (x as i32, y as i32);
                }

                target_position = (x as i32, y as i32);
            }
            map.insert((x as i32, y as i32), tile);
        }
    }
    Ok(Map {
        map,
        starting_position,
        target_position,
    })
}
