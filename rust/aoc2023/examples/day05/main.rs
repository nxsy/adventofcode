use std::{collections::BTreeMap, ops::Range};

use anyhow::Result;
use strum_macros::EnumString;
use thiserror::Error;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64 as nom_u64},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MapEntry {
    source: u64,
    destination: u64,
    length: u64,
    source_range: MapRange,
    destination_range: MapRange,
}

impl MapEntry {
    fn new(source: u64, destination: u64, length: u64) -> Self {
        Self {
            source,
            destination,
            length,
            source_range: MapRange(source..(source + length)),
            destination_range: MapRange(destination..(destination + length)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Data {
    seeds: Vec<u64>,
    maps: BTreeMap<(Resource, Resource), Map>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MapRange(Range<u64>);

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0.start, self.0.end).cmp(&(other.0.start, other.0.end))
    }
}

impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OverlapResult {
    before: Option<MapRange>,
    intersection: Option<MapRange>,
    after: Option<MapRange>,
}

impl MapRange {
    fn overlap(&self, other: &Self) -> OverlapResult {
        let before = self.0.start..(other.0.start.min(self.0.end));
        let intersection = (self.0.start.max(other.0.start))..(self.0.end.min(other.0.end));
        let after = (self.0.start.max(other.0.end))..self.0.end;

        OverlapResult {
            before: if before.clone().count() > 0 {
                Some(Self(before))
            } else {
                None
            },
            intersection: if intersection.clone().count() > 0 {
                Some(Self(intersection))
            } else {
                None
            },
            after: if after.clone().count() > 0 {
                Some(Self(after))
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Map(Vec<MapEntry>);

impl Map {
    fn convert(&self, value: u64) -> u64 {
        let mut value = value;
        for entry in self.0.iter() {
            if value >= entry.source && value < entry.source + entry.length {
                value = entry.destination + (value - entry.source);
                break;
            }
        }
        value
    }

    fn convert_ranges(&self, values: &[MapRange]) -> Vec<MapRange> {
        let mut out_ranges = Vec::new();

        let mut to_process = values.to_vec();

        for entry in self.0.iter() {
            let mut process_next_entry = Vec::new();
            while let Some(r) = to_process.pop() {
                let overlap = r.overlap(&entry.source_range);
                if let Some(before) = overlap.before {
                    process_next_entry.push(before);
                }
                if let Some(intersection) = overlap.intersection {
                    let start_value = entry.destination + (intersection.0.start - entry.source);
                    let end_value = entry.destination + (intersection.0.end - entry.source);
                    out_ranges.push(MapRange(start_value..end_value));
                }
                if let Some(after) = overlap.after {
                    process_next_entry.push(after);
                }
            }
            to_process = process_next_entry;
        }

        out_ranges.append(&mut to_process);
        out_ranges
    }
}

fn parse_resource(input: &str) -> IResult<&str, Resource> {
    let (input, resource_name) = alt((
        tag("seed"),
        tag("soil"),
        tag("fertilizer"),
        tag("water"),
        tag("light"),
        tag("temperature"),
        tag("humidity"),
        tag("location"),
    ))(input)?;
    Ok((input, resource_name.parse().unwrap()))
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(tag(" "), nom_u64),
        tuple((line_ending, line_ending)),
    )(input)?;

    let mut maps = BTreeMap::new();

    let (input, map_data) = separated_list1(
        tuple((line_ending, line_ending)),
        separated_pair(
            separated_pair(
                parse_resource,
                tag("-to-"),
                terminated(parse_resource, tag(" map:")),
            ),
            line_ending,
            separated_list1(
                line_ending,
                separated_pair(nom_u64, space1, separated_pair(nom_u64, space1, nom_u64)),
            ),
        ),
    )(input)?;

    for m in map_data {
        maps.insert(
            (m.0 .0, m.0 .1),
            Map(m
                .1
                .iter()
                .map(|(d, (s, l))| MapEntry::new(*s, *d, *l))
                .collect()),
        );
    }
    Ok((input, Data { seeds, maps }))
}

fn part1(input: &str) -> Result<String> {
    let (_, data) = parse_input(input).unwrap();
    let seed_to_soil = data.maps.get(&(Resource::Seed, Resource::Soil)).unwrap();
    let soil_to_fertilizer = data
        .maps
        .get(&(Resource::Soil, Resource::Fertilizer))
        .unwrap();
    let fertilizer_to_water = data
        .maps
        .get(&(Resource::Fertilizer, Resource::Water))
        .unwrap();
    let water_to_light = data.maps.get(&(Resource::Water, Resource::Light)).unwrap();
    let light_to_temperature = data
        .maps
        .get(&(Resource::Light, Resource::Temperature))
        .unwrap();
    let temperature_to_humidity = data
        .maps
        .get(&(Resource::Temperature, Resource::Humidity))
        .unwrap();
    let humidity_to_location = data
        .maps
        .get(&(Resource::Humidity, Resource::Location))
        .unwrap();

    let result = data
        .seeds
        .iter()
        .map(|&s| {
            let soil = seed_to_soil.convert(s);
            let fertilizer = soil_to_fertilizer.convert(soil);
            let water = fertilizer_to_water.convert(fertilizer);
            let light = water_to_light.convert(water);
            let temperature = light_to_temperature.convert(light);
            let humidity = temperature_to_humidity.convert(temperature);
            humidity_to_location.convert(humidity)
        })
        .min()
        .unwrap();
    Ok(result.to_string())
}

fn part2(input: &str) -> Result<String> {
    let (_, data) = parse_input(input).unwrap();
    let seed_to_soil = data.maps.get(&(Resource::Seed, Resource::Soil)).unwrap();
    let soil_to_fertilizer = data
        .maps
        .get(&(Resource::Soil, Resource::Fertilizer))
        .unwrap();
    let fertilizer_to_water = data
        .maps
        .get(&(Resource::Fertilizer, Resource::Water))
        .unwrap();
    let water_to_light = data.maps.get(&(Resource::Water, Resource::Light)).unwrap();
    let light_to_temperature = data
        .maps
        .get(&(Resource::Light, Resource::Temperature))
        .unwrap();
    let temperature_to_humidity = data
        .maps
        .get(&(Resource::Temperature, Resource::Humidity))
        .unwrap();
    let humidity_to_location = data
        .maps
        .get(&(Resource::Humidity, Resource::Location))
        .unwrap();

    let ranges: Vec<MapRange> = data
        .seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            MapRange(start..(start + length))
        })
        .collect();

    let soil = seed_to_soil.convert_ranges(&ranges);
    let fertilizer = soil_to_fertilizer.convert_ranges(&soil);
    let water = fertilizer_to_water.convert_ranges(&fertilizer);
    let light = water_to_light.convert_ranges(&water);
    let temperature = light_to_temperature.convert_ranges(&light);
    let humidity = temperature_to_humidity.convert_ranges(&temperature);
    let location = humidity_to_location.convert_ranges(&humidity);

    let lowest_location = location.iter().map(|r| r.0.start).min().unwrap();
    Ok(lowest_location.to_string())
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
        let expected = "35";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "46";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
