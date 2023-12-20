use std::collections::{BTreeMap, BTreeSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::Config;
use crate::Module;

fn parse_module(input: &'static str) -> IResult<&str, Module> {
    alt((
        tag("broadcaster").map(|_| Module::Broadcaster),
        preceded(tag("%"), alpha1).map(Module::FlipFlop),
        preceded(tag("&"), alpha1).map(Module::Conjunction),
    ))(input)
}

pub(crate) fn parse(input: &'static str) -> IResult<&str, Config> {
    let (input, modules) = all_consuming(separated_list1(
        line_ending,
        separated_pair(
            parse_module,
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
    ))(input)?;
    let mut module_name_to_module = BTreeMap::new();
    for module in &modules {
        match module.0 {
            Module::Broadcaster => {}
            Module::FlipFlop(name) | Module::Conjunction(name) => {
                module_name_to_module.insert(name, module.0);
            }
            Module::Untyped(_) | Module::Button => {
                unreachable!();
            }
        }
    }

    let mut untyped = BTreeSet::new();
    let mut modules: BTreeMap<Module, Vec<Module>> = modules
        .into_iter()
        .map(|(module, outputs)| {
            (
                module,
                outputs
                    .into_iter()
                    .map(|name| {
                        if let Some(x) = module_name_to_module.get(name) {
                            *x
                        } else {
                            untyped.insert(Module::Untyped(name));
                            Module::Untyped(name)
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    modules.insert(Module::Button, vec![Module::Broadcaster]);

    for module in untyped {
        module_name_to_module.insert(module.name(), module);
        modules.insert(module, vec![]);
    }

    let reverse_modules: BTreeMap<Module, Vec<Module>> =
        modules
            .iter()
            .fold(BTreeMap::new(), |mut acc, (module, outputs)| {
                for output in outputs {
                    acc.entry(*output).or_default().push(*module);
                }
                acc
            });

    Ok((
        input,
        Config {
            modules,
            module_name_to_module,
            reverse_modules,
        },
    ))
}
