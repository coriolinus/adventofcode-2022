use aoclib::parse;
use itertools::Itertools;
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

// Notes in this implementation:
//
// We could simplify things by going immediately to a bitmask instead of keeping a (fairly large)
// count of items. We choose not to do this because of the heuristic that in general we want to
// parse AoC inputs into some form which allows us to reconstruct the entire input. As inputs
// can contain multiple instances of items, we keep the count. While it turned out that part 2
// didn't need us to use the counts this time, the heuristic is still valuable.
//
// Priorities are 1-indexed, and we want to sum them often. To keep the downstream implementation simple,
// we waste 32 bits of space for slot 0, and then work directly on the priority set.
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

// This is unicode-safe. We could operate on the implicit rule that AoC input is always ASCII,
// and performance would increase, but it's more fun to work in a way which supports more
// challenging inputs.
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

fn find_badge(group: &[Priorities]) -> Result<u8, Error> {
    let common_items = group
        .iter()
        .fold(!0, |accumulator, item| accumulator & item.as_flags());
    if common_items.count_ones() != 1 {
        return Err(Error::WrongCountCommonItems(common_items.count_ones()));
    }
    for idx in 0..(u64::BITS) {
        if common_items & (1 << idx) != 0 {
            return Ok(idx as u8);
        }
    }
    unreachable!("we can exhaustively search a u64");
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
    let mut badge_sum = 0;
    for chunk in parse::<String>(input)?.chunks(3).into_iter() {
        let (left, mid, right) = chunk.collect_tuple().ok_or(Error::IncompleteGroup)?;
        let priorities = [left.parse::<Priorities>()?, mid.parse()?, right.parse()?];
        let badge = find_badge(&priorities)?;
        badge_sum += badge as u32;
    }
    println!("sum of priorities of group badges: {badge_sum}");
    Ok(())
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
    #[error("incomplete group")]
    IncompleteGroup,
    #[error("expected 1 common item; got {0}")]
    WrongCountCommonItems(u32),
}
