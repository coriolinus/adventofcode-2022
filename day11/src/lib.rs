use std::path::Path;

use crate::troop::Troop;

mod models;
mod parse;
mod troop;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut troop = Troop::new(parse::parse(input)?);
    for _ in 0..20 {
        troop.round();
    }
    let monkey_business: u32 = troop
        .active_monkeys(2)
        .into_iter()
        .map(|monkey| monkey.inspect_count)
        .product();
    println!("monkey business: {monkey_business}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    unimplemented!("input file: {:?}", input)
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
