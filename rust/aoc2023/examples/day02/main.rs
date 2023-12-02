//! Day 02

use anyhow::Result;
use clap::Parser;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Input {
    Example,
    Final,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    part: Part,
    #[arg(long)]
    input: Input,
}

#[derive(Debug, strum::EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::enum_variant_names)]
#[strum(serialize_all = "snake_case")]
enum Ball {
    Blue,
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn new() -> Self {
        Bag {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn contained_in_bag(&self, other: &Bag) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn minimum_combined_bag(&self, other: &Bag) -> Bag {
        Bag {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl From<Vec<(u32, Ball)>> for Bag {
    fn from(value: Vec<(u32, Ball)>) -> Self {
        let mut bag = Bag::new();
        for (count, ball) in value {
            match ball {
                Ball::Blue => bag.blue += count,
                Ball::Red => bag.red += count,
                Ball::Green => bag.green += count,
            }
        }
        bag
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Game {
    game_number: u32,
    bags: Vec<Bag>,
}

fn parse_ball(input: &str) -> IResult<&str, Ball> {
    let (input, ball_color) = alt((tag("blue"), tag("red"), tag("green")))(input)?;
    Ok((input, ball_color.parse::<Ball>().unwrap()))
}

fn parse_bag(input: &str) -> IResult<&str, Bag> {
    let (input, balls) = separated_list1(
        tag(", "),
        separated_pair(map_res(digit1, str::parse::<u32>), tag(" "), parse_ball),
    )(input)?;
    Ok((input, Bag::from(balls)))
}

fn line_to_games(input: &str) -> Result<Game> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (_, (game_number, bags)) = separated_pair(
        preceded(tag("Game "), map_res(digit1, str::parse::<u32>)),
        tag(": "),
        separated_list1(tag("; "), parse_bag),
    )(input).map_err(|e| e.to_owned())?;
    Ok(Game { game_number, bags })
}

fn part1(file_data: &str) -> Result<()> {
    let games = file_data
        .lines()
        .map(line_to_games)
        .collect::<Result<Vec<Game>>>()?;

    let original_bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let game_sum = games
        .iter()
        .filter(|g| g.bags.iter().all(|b| b.contained_in_bag(&original_bag)))
        .map(|g| g.game_number)
        .sum::<u32>();

    println!("Game sum: {}", game_sum);
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let games = file_data
        .lines()
        .map(line_to_games)
        .collect::<Result<Vec<Game>>>()?;

    let power_sum: u32 = games
        .iter()
        .map(|g| {
            g.bags
                .iter()
                .fold(Bag::new(), |combined_bag, b| combined_bag.minimum_combined_bag(b))
                .power()
        })
        .sum();

    println!("Power sum: {}", power_sum);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::Example => include_str!("example_input.txt"),
        Input::Final => include_str!("input.txt"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
