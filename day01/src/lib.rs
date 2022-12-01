use aoclib::input::parse_newline_sep;
use std::{path::Path, str::FromStr};

struct Elf {
    calories: Vec<u32>,
}

impl FromStr for Elf {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Error::MalformedInput)?;

        Ok(Self { calories })
    }
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let elves: Vec<Elf> = parse_newline_sep(input)?.collect();
    let careful_elf = elves
        .iter()
        .max_by_key(|elf| elf.total_calories())
        .ok_or(Error::NoSolution)?;
    println!("most calories: {}", careful_elf.total_calories());
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut elves: Vec<Elf> = parse_newline_sep(input)?.collect();
    elves.sort_by_cached_key(|elf| std::cmp::Reverse(elf.total_calories()));
    let top_3_total_calories: u32 = elves.iter().take(3).map(|elf| elf.total_calories()).sum();
    println!("top 3 calories: {top_3_total_calories}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
    #[error("malformed calorie input")]
    MalformedInput,
}
