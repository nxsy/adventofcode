//! Day 21

use adventofcode2022::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u64 as nom_u64},
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
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

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn perform(&self, op1: u64, op2: u64) -> u64 {
        match self {
            Operation::Add => op1 + op2,
            Operation::Subtract => op1 - op2,
            Operation::Multiply => op1 * op2,
            Operation::Divide => op1 / op2,
        }
    }
}

#[derive(Debug)]
enum Expression {
    Constant(u64),
    Expression(Operation, String, String),
    Variable,
}

#[derive(Debug)]
struct Monkey {
    name: String,
    expr: Expression,
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    if let Ok((input, value)) = nom_u64::<_, Error<_>>(input) {
        Ok((input, Expression::Constant(value)))
    } else {
        let (input, (op1, operator, op2)) = tuple((
            alpha1,
            delimited(
                tag(" "),
                alt((tag("+"), tag("-"), tag("*"), tag("/"))),
                tag(" "),
            ),
            alpha1,
        ))(input)?;

        let operator = match operator {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => unreachable!(),
        };
        Ok((
            input,
            Expression::Expression(operator, op1.to_string(), op2.to_string()),
        ))
    }
}

fn parse_line(input: &str) -> IResult<&str, Monkey> {
    let (input, (name, expr)) = tuple((terminated(alpha1, tag(": ")), parse_expression))(input)?;
    let name = name.to_string();
    Ok((input, Monkey { name, expr }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    all_consuming(separated_list1(line_ending, parse_line))(input)
}

fn part1(file_data: &str) -> Result<()> {
    let (_, monkeys) = parse_input(file_data).unwrap();

    let mut monkeys = monkeys
        .into_iter()
        .map(|m| (m.name, m.expr))
        .collect::<HashMap<_, _>>();
    let mut constants = HashSet::new();

    for m in &monkeys {
        if let Expression::Constant(_) = m.1 {
            constants.insert(m.0.to_string());
        }
    }

    loop {
        let mut new_vals = HashMap::new();
        for m in monkeys.iter() {
            if let Expression::Expression(operator, op1, op2) = m.1 {
                if let (Expression::Constant(op1), Expression::Constant(op2)) =
                    (&monkeys[op1], &monkeys[op2])
                {
                    new_vals.insert(
                        m.0.to_string(),
                        Expression::Constant(operator.perform(*op1, *op2)),
                    );
                    constants.insert(m.0.clone());
                }
            }
        }
        monkeys.extend(new_vals);
        if constants.contains("root") {
            break;
        }
    }

    dbg!(&monkeys["root"]);

    Ok(())
}

fn print_expression(expressions: &HashMap<String, Expression>, name: &str) -> String {
    match &expressions[name] {
        Expression::Constant(v) => v.to_string(),
        Expression::Expression(operator, op1, op2) => {
            format!(
                "({} {} {})",
                print_expression(expressions, op1),
                match operator {
                    Operation::Add => "+",
                    Operation::Subtract => "-",
                    Operation::Multiply => "*",
                    Operation::Divide => "/",
                },
                print_expression(expressions, op2),
            )
        }
        Expression::Variable => "x".to_string(),
    }
}

fn part2(file_data: &str) -> Result<()> {
    let (_, monkeys) = parse_input(file_data).unwrap();

    let mut monkeys = monkeys
        .into_iter()
        .map(|m| (m.name, m.expr))
        .collect::<HashMap<_, _>>();
    monkeys.insert("humn".to_string(), Expression::Variable);
    let mut constants = HashSet::new();

    for m in &monkeys {
        if let Expression::Constant(_) = m.1 {
            constants.insert(m.0.to_string());
        }
    }

    loop {
        let mut new_vals = HashMap::new();
        for m in monkeys.iter() {
            if let Expression::Expression(operator, op1, op2) = m.1 {
                if let (Expression::Constant(op1), Expression::Constant(op2)) =
                    (&monkeys[op1], &monkeys[op2])
                {
                    new_vals.insert(
                        m.0.to_string(),
                        Expression::Constant(operator.perform(*op1, *op2)),
                    );
                    constants.insert(m.0.clone());
                }
            }
        }
        if new_vals.is_empty() {
            break;
        }
        monkeys.extend(new_vals);
    }

    let Expression::Expression(_, op1, op2) = &monkeys["root"] else { unreachable!() };
    let op1 = print_expression(&monkeys, op1);
    let op2 = print_expression(&monkeys, op2);
    println!("{} = {}", op1, op2);

    //TODO: equation solver!

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::ExampleInput => include_str!("example_input"),
        Input::FinalInput => include_str!("input"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
