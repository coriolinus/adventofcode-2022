use aoclib::parse;
use parse_display::{Display, FromStr};
use std::path::Path;

#[derive(Debug, Clone, Copy, FromStr, Display)]
#[display("{low}-{high}")]
struct Assignment {
    low: u32,
    high: u32,
}

impl Assignment {
    fn fully_contains(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn overlaps_low(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.low
    }
}

#[derive(Debug, Clone, Copy, FromStr, Display)]
#[display("{left},{right}")]
struct Pair {
    left: Assignment,
    right: Assignment,
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let fully_contained = parse::<Pair>(input)?
        .filter(|pair| {
            pair.left.fully_contains(&pair.right) || pair.right.fully_contains(&pair.left)
        })
        .count();
    println!("fully contained: {fully_contained}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let overlaps = parse::<Pair>(input)?
        .filter(|pair| pair.left.overlaps_low(&pair.right) || pair.right.overlaps_low(&pair.left))
        .count();
    println!("overlaps: {overlaps}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
