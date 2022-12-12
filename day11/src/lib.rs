use std::{ffi::OsStr, path::Path};

use crate::troop::Troop;

mod models;
mod parse;
mod troop;

pub(crate) fn env_is_set(key: impl AsRef<OsStr>) -> bool {
    !std::env::var(key).unwrap_or_default().is_empty()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut troop = Troop::new(parse::parse(input)?, true);
    for _ in 0..20 {
        troop.round();
    }
    let monkey_business: u32 = troop
        .active_monkeys(2)
        .into_iter()
        .map(|monkey| monkey.inspect_count)
        .product();
    println!("monkey business (pt. 1): {monkey_business}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut troop = Troop::new(parse::parse(input)?, false);
    for round in 0..10_000 {
        if env_is_set("INSPECTION_SUMMARIES") && (round == 1 || round == 20 || round % 1000 == 0) {
            eprintln!("After round {round}:");
            for monkey in troop.iter() {
                eprintln!(
                    "Monkey {} inspected items {} times",
                    monkey.id.0, monkey.inspect_count
                );
            }
        }
        troop.round();
    }
    if env_is_set("INSPECTION_SUMMARIES") {
        eprintln!("After round 10000:");
        for monkey in troop.iter() {
            eprintln!(
                "Monkey {} inspected items {} times",
                monkey.id.0, monkey.inspect_count
            );
        }
    }
    let monkey_business: u64 = troop
        .active_monkeys(2)
        .into_iter()
        .map(|monkey| monkey.inspect_count as u64)
        .product();
    println!("monkey business (pt. 2): {monkey_business}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
    #[error("malformed input: monkey content before builder")]
    MissingContent,
    #[error("malformed input: builder error")]
    MonkeyBuilder(#[from] models::MonkeyBuilderError),
    #[error("malformed input: misplaced monkey. Expected monkey {0}, got {1}")]
    MisplacedMonkey(usize, usize),
}
