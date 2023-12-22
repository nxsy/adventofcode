use std::collections::{BTreeMap, BTreeSet};

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Plot,
    Rock,
}

#[derive(Debug)]
struct Map {
    tiles: BTreeMap<(i32, i32), Tile>,
    extents: (i32, i32, i32, i32),
    starting_position: (i32, i32),
}

fn parse(input: &str) -> Result<Map> {
    let mut tiles = BTreeMap::new();
    let mut starting_position = (0, 0);
    let mut extents = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for (y, line) in input.lines().enumerate() {
        (extents.1, extents.3) = (extents.1.min(y as i32), extents.3.max(y as i32));
        for (x, c) in line.chars().enumerate() {
            (extents.0, extents.2) = (extents.0.min(x as i32), extents.2.max(x as i32));
            match c {
                '.' | 'S' => {
                    tiles.insert((x as i32, y as i32), Tile::Plot);
                    if c == 'S' {
                        starting_position = (x as i32, y as i32);
                    }
                }
                '#' => {
                    tiles.insert((x as i32, y as i32), Tile::Rock);
                }
                _ => bail!("invalid character at {}, {}", x, y),
            }
        }
    }
    Ok(Map {
        tiles,
        extents,
        starting_position,
    })
}

fn solve_one_map(start: (i32, i32), num_steps: usize, map: &Map) -> usize {
    let mut positions = BTreeSet::from([start]);
    for _ in 0..num_steps {
        let mut new_positions = BTreeSet::new();
        for position in positions.clone() {
            for (x, y) in &[
                (position.0 - 1, position.1),
                (position.0 + 1, position.1),
                (position.0, position.1 - 1),
                (position.0, position.1 + 1),
            ] {
                if let Some(Tile::Plot) = map.tiles.get(&(*x, *y)) {
                    new_positions.insert((*x, *y));
                }
            }
        }
        positions = new_positions;
    }

    positions.len()
}

fn part1(input: &str, num_steps: usize) -> Result<String> {
    let map = parse(input)?;

    let result = solve_one_map(map.starting_position, num_steps, &map);
    Ok(result.to_string())
}

fn part2(input: &str, num_steps: usize) -> Result<String> {
    let map = parse(input)?;

    let size = (map.extents.2 - map.extents.0 + 1) as usize;

    /*

    Over enough steps, we expect to see a diamond shape centered around the
    starting position.

    Each quadrant of this diamond will be subtly different at only its edges
    as the interior tilemaps will be fully explored.

    The input has several attributes that allow us to make the assumption
    that we always first enter each tilemap from the corner or middle of an
    edge, and always have the same remaining steps as we reach each tilemap
    based on the Manhattan distance from the starting tilemap:

    1. The starting position is always in the center of the tilemap (and thus
       the extents are always odd).
    2. The input is always a square.
    3. There are clear rows and columns aligned with the starting position.
    4. The are clear rows and columns on the edges.

    The means we can always reach the center of the tilemap in half the size
    from any midpoint (minus one), and move from any tilemap midpoint or
    corner to the neighbouring tilemap in the same position in size steps.

    This essentially means it's always optimal to reach any midpoint of edge
    or corner of a tilemap or center from any of these positions in another
    tilemap via going along the empty rows/columns.  Since we start in one of
    these positions, we can always reach any other position in some known
    multiple of size steps to get to the right tilemap, and then some known
    number of multiples of half-size steps (minus one).

    */

    // Assumptions above
    assert_eq!(size, (map.extents.3 - map.extents.1 + 1) as usize);
    assert_eq!(size % 2, 1);
    assert_eq!(map.starting_position.0, map.starting_position.1);
    assert_eq!(map.starting_position.0, (size / 2) as i32);

    // Later assumption: The number of steps is some multiple of the size plus
    // half the size.
    assert_eq!(num_steps % size, size / 2);

    // General assumption: We're dealing with a large number of steps, so we
    // always have corners + each of the diagonal types:
    assert!(num_steps > size * 10);

    /*

    The interior tilemaps are fully explored, distinguished only by whether
    we moved an odd or even number of steps from the starting position to
    reach them.

    Given the size of the map is odd, that means we have a checkerboard
    pattern of "odd tilemaps" and "even tilemaps", since in-tilemap
    checkerboard is inverted, as shown:

    +---+---+
    |121|212|
    |212|121|
    |121|212|
    +---+---+
    |212|121|
    |121|212|
    |212|121|
    +---+---+

    */

    let full_odd_positions = solve_one_map(map.starting_position, size / 2 * 2 + 1, &map);
    let full_even_positions = solve_one_map(map.starting_position, size / 2 * 2, &map);

    // The number of interior tilemaps along the axes
    let interior_tilemaps_width = (num_steps / size).saturating_sub(1);

    /*

    The progression in terms of odd/even tilemaps by width is like this:

    width: 0, odd:  1, even:  0                   2
    width: 1, odd:  1, even:  4           1      212
    width: 2, odd:  9, even:  4      2   121    21212
    width: 3, odd:  9, even: 16   1 212 12121  2121212
    width: 4, odd: 25, even: 16      2   121    21212
    width: 5, odd: 25, even: 36           1      212
                                                  2

    So odd is the square of width rounded down to nearest even number + 1
    So even is the square of (width + 1) rounded down to nearest even number

    */

    fn round_to_even(n: usize) -> usize {
        n & !1
    }

    let num_odd_maps = if num_steps > size {
        (round_to_even(interior_tilemaps_width) + 1).pow(2)
    } else {
        0
    };
    let num_even_maps = (round_to_even(interior_tilemaps_width + 1)).pow(2);

    /*
    Besides this fully-explored interior, we have four corners at cardinal
    directions, and diagonals between those corners that are partially full -
    cycling between the "small diagonals" and "big diagonals" that cycle
    between the corners.

    C: Corner
    F: Full
    S: Small diagonal
    B: Big diagonal
    E: Empty

    +E----+S----+C-^--+S----+E----+
    |     |     | /■\ |     |     |
    |     |     |/■■■\|     |     |
    |     |     /■■■■■\     |     |
    |     |    /|■■■■■|\    |     |
    |     |   /■|■■■■■|■\   |     |
    +S----+B-/--+F----+B-\--+S----+
    |     | /■■■|■■■■■|■■■\ |     |
    |     |/■■■■|■■■■■|■■■■\|     |
    |     /■■■■■|■■■■■|■■■■■\     |
    |    /|■■■■■|■■■■■|■■■■■|\    |
    |   /■|■■■■■|■■■■■|■■■■■|■\   |
    +B-/--+F----+F----+F----+B-\--+
    | /■■■|■■■■■|■■■■■|■■■■■|■■■\ |

    */

    /*

    The number of steps is a multiple of the size, plus half the size.  Half
    the size is used to reach the edge of the center tilemap (the starting
    row/column is empty) and it takes a multiple of size to reach the corner
    tilemap, so we have a size (minus one) left to explore the corner
    tilemap.

    This will allow us to reach a single tile in the midpoint of the edge
    opposite the one we entered from (again, the starting row/column is
    empty).

    */

    // Always enter corners from the midpoint of the edge closest to the center
    let diamond_corners_starting_positions = [
        (map.extents.0, map.starting_position.1),
        (map.extents.2, map.starting_position.1),
        (map.starting_position.0, map.extents.1),
        (map.starting_position.0, map.extents.3),
    ];

    let corner_sum = diamond_corners_starting_positions
        .iter()
        .map(|start| solve_one_map(*start, size - 1, &map))
        .sum::<usize>();

    /*

    We handle each quadrant of the diamond separately for the diagonals,
    since the diagonals will be entered from different corners in each
    quadrant.

    Within the quadrant, we will cycle between the small diagonals and big,
    diagonals, but we will have small diagonals next to both corners.

    CS
    XBS
    X BS
    X  BS
    +XXXC

    width: 3, small: 4, big: 3
    */

    let num_small_diagonals = interior_tilemaps_width + 1;
    let num_big_diagonals = interior_tilemaps_width;

    // Always enter diagonals from the corner closest to the center
    let diagonals_starting_positions = [
        (map.extents.0, map.extents.3),
        (map.extents.0, map.extents.1),
        (map.extents.2, map.extents.1),
        (map.extents.2, map.extents.3),
    ];

    /*

    The diagonals closest to the corners are smaller diagonals, and the other
    smaller diagonals are identical to these (since we move multiples of size
    along each axis to reach the same starting corner).

    Since we reach the corner at the midpoint of the edge closest to the
    center, we can reach the corner of the diagonal in half the size steps,
    and this is always optimal due to the Manhattan distance and the empty
    rows and columns on the edges.

    This leaves us with half the size steps to explore the small diagonal.

    */
    let small_diagonal_sum = diagonals_starting_positions
        .iter()
        .map(|start| solve_one_map(*start, size / 2 - 1, &map))
        .sum::<usize>();

    /*

    The diagonals two away from the corners are bigger diagonals, and the
    other bigger diagonals are identical to these, for the same reason as
    the smaller diagonals.

    We can reach the closest corner of the big diagonal by moving one size
    closer to the starting position from a small diagonal - ie, if we're at
    the top corner's right small diagonal, we could have taken size fewer
    steps upwards to reach the big diagonal below it).

    So, in addition to the half-size steps (minus 1) the small diagonal had,
    we also have an additional size steps to explore from the corner of the
    big diagonal.

    */
    let big_diagonal_sum = diagonals_starting_positions
        .iter()
        .map(|start| solve_one_map(*start, size * 3 / 2 - 1, &map))
        .sum::<usize>();

    /*

    And that's it:
    * The fully-explored interior tilemaps (odd and even varieties)
    * The four corners
    * The small diagonals
    * The big diagonals

     */
    let total = full_odd_positions * num_odd_maps
        + full_even_positions * num_even_maps
        + corner_sum
        + small_diagonal_sum * num_small_diagonals
        + big_diagonal_sum * num_big_diagonals;

    Ok(total.to_string())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let part1_result = match part1(input, 64) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part1: {}", part1_result);
    let part2_result = match part2(input, 26501365) {
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
        let expected = "16";
        let actual = part1(file_data, 6)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
