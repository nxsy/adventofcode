//! Day 15
//!
//! Always useful to read.  Manhattan distance is way less work than the
//! alternative.  Guess which one I wrote first?  Guess whether that wrong
//! approach works for the example input!
//!
//! For part 1, I almost tried to do something fancy with ranges instead
//! of just populating the entire covered space in a set.  But fancy is
//! slow to write and complicated.
//!
//! For part 2, if there's literally only one place not covered by the
//! sensors, it has to be exactly one away from the range of a sensor.
//! (Was not looking forward to trying to do space partitioning.)

use adventofcode2022::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    part: Part,
    #[arg(long)]
    input: Input,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos(i64, i64);

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn parse_input(input: &str) -> IResult<&str, HashMap<Pos, Pos>> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let (input, pairs) = separated_list1(
        line_ending,
        tuple((
            preceded(tag("Sensor at x="), nom_i64),
            preceded(tag(", y="), nom_i64),
            preceded(tag(": closest beacon is at x="), nom_i64),
            preceded(tag(", y="), nom_i64),
        )),
    )(input)?;
    Ok((
        input,
        pairs
            .into_iter()
            .map(|(sx, sy, bx, by)| (Pos(sx, sy), Pos(bx, by)))
            .collect::<HashMap<_, _>>(),
    ))
}

fn part1(file_data: &str, line_no: i64) -> Result<()> {
    let (_, sensor_closest_beacons) = parse_input(file_data).unwrap();
    let mut covered = HashSet::new();
    let mut beacons = HashSet::new();
    for (spos, bpos) in sensor_closest_beacons.iter() {
        if bpos.1 == line_no {
            beacons.insert(bpos.0);
        }
        let sensor_clear_distance = spos.manhattan_distance(bpos);
        let sensor_to_line_no_distance = (spos.1 - line_no).abs();
        if sensor_to_line_no_distance > sensor_clear_distance {
            continue;
        }

        let x_delta = sensor_clear_distance - sensor_to_line_no_distance;
        let x_start = spos.0 - x_delta;
        let x_end = spos.0 + x_delta;

        for x in x_start..=x_end {
            covered.insert(x);
        }
    }
    let covered_without_beacons: HashSet<_> = covered.difference(&beacons).collect();
    dbg!(covered_without_beacons.len());

    Ok(())
}

fn within_sensor_range(pos: Pos, sensors: &Vec<(Pos, i64)>) -> bool {
    for &(spos, sd) in sensors {
        let d = pos.manhattan_distance(&spos);
        if d <= sd {
            return true;
        }
    }
    false
}

fn part2(file_data: &str, range_min: i64, range_max: i64) -> Result<()> {
    let (_, sensor_closest_beacons) = parse_input(file_data).unwrap();

    let sensors_with_distances = sensor_closest_beacons
        .into_iter()
        .map(|(spos, bpos)| (spos, spos.manhattan_distance(&bpos)))
        .collect::<Vec<_>>();

    'search: for &(spos, d) in &sensors_with_distances {
        let d = d + 1;
        let (sx, sy) = (spos.0, spos.1);
        let explore = [
            ((sx, sy + d), (1, -1), (sx + d, sy)),
            ((sx + d, sy), (-1, -1), (sx, sy - d)),
            ((sx, sy - d), (-1, 1), (sx - d, sy)),
            ((sx - d, sy), (1, 1), (sx, sy + d)),
        ];
        for (mut pos, dpos, final_pos) in explore {
            while pos != final_pos {
                if (pos.0 >= range_min
                    && pos.0 <= range_max
                    && pos.1 >= range_min
                    && pos.1 <= range_max)
                    && !within_sensor_range(Pos(pos.0, pos.1), &sensors_with_distances)
                {
                    println!("{pos:?}!!!");
                    let tuning_frequency = (pos.0 * 4000000) + pos.1;
                    dbg!(tuning_frequency);
                    break 'search;
                }
                pos.0 += dpos.0;
                pos.1 += dpos.1;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let (file_data, line_no, range_min, range_max) = match args.input {
        Input::ExampleInput => (include_str!("example_input"), 10, 0, 20),
        Input::FinalInput => (include_str!("input"), 2_000_000, 0, 4_000_000),
    };
    match args.part {
        Part::Part1 => part1(file_data, line_no),
        Part::Part2 => part2(file_data, range_min, range_max),
    }
}
