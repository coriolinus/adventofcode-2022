use aoclib::parse;
use std::path::Path;

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

fn parse_to_priorities(s: impl AsRef<str>) -> Result<(Priorities, Priorities), Error> {
    let s = s.as_ref();
    let total = s.chars().count();
    let side_size = total / 2;
    if side_size * 2 != total {
        return Err(Error::OddNumber);
    }

    let mut left = Priorities::default();
    let mut right = Priorities::default();

    for (idx, ch) in s.chars().enumerate() {
        let priority = priority_of(ch)?;
        if idx < side_size {
            left.insert(priority);
        } else {
            right.insert(priority);
        }
    }
    debug_assert_eq!(
        left.0.iter().map(|x| *x as u32).sum::<u32>(),
        right.0.iter().map(|x| *x as u32).sum::<u32>(),
        "must have same number of items in both sides of rucksack"
    );

    Ok((left, right))
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut mutual_priority_sum = 0;
    for rucksack_contents in parse::<String>(input)? {
        let (left, right) = parse_to_priorities(rucksack_contents)?;
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
