use std::path::Path;

mod models;
mod parse;
mod troop;

pub fn part1(input: &Path) -> Result<(), Error> {
    let monkeys = parse::parse(input)?;
    println!("parsed {} monkeys", monkeys.len());
    unimplemented!("input file: {:?}", input)
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
