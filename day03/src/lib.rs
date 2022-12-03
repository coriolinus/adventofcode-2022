use aoclib::parse;
use std::{path::Path, str::FromStr};

fn priority_of(value: char) -> Result<u8, Error> {
    if ('a'..='z').contains(&value) {
        Ok(value as u8 - b'a' + 1)
    } else if ('A'..='Z').contains(&value) {
        Ok(value as u8 - b'A' + 1 + 26)
    } else {
        Err(Error::MalformedPriority(value))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Priorities([u32; 53]);

impl Default for Priorities {
    fn default() -> Self {
        Self([0; 53])
    }
}

impl FromStr for Priorities {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Self::default();
        for ch in s.chars() {
            let priority = priority_of(ch)?;
            p.insert(priority);
        }
        Ok(p)
    }
}

impl Priorities {
    fn insert(&mut self, priority: u8) {
        self.0[priority as usize] += 1;
    }

    fn as_flags(&self) -> u64 {
        let mut flags = 0;
        for (idx, count) in self.0.iter().copied().enumerate() {
            if count > 0 {
                flags |= 1 << idx;
            }
        }
        flags
    }
}

fn halve_string(mut s: String) -> Result<(String, String), Error> {
    let total = s.chars().count();
    let half = total / 2;
    if half * 2 != total {
        return Err(Error::OddNumber);
    }
    let split_point = s
        .char_indices()
        .nth(half)
        .expect("taking the next char at the half point doesn't exhaust the string")
        .0;
    let right = s.split_off(split_point);
    Ok((s, right))
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut mutual_priority_sum = 0;
    for rucksack_contents in parse::<String>(input)? {
        let (left, right) = halve_string(rucksack_contents)?;
        let left: Priorities = left.parse()?;
        let right: Priorities = right.parse()?;
        let intersection = left.as_flags() & right.as_flags();
        for idx in 0..(u64::BITS) {
            if intersection & (1 << idx) != 0 {
                mutual_priority_sum += idx;
            }
        }
    }
    println!("mutual priority sum: {mutual_priority_sum}");
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
    #[error("malformed priority: {0}")]
    MalformedPriority(char),
    #[error("odd number of items in rucksack")]
    OddNumber,
}
