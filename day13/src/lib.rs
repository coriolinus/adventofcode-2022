use aoclib::input::parse_newline_sep;
use derive_more::From;
use std::{cmp::Ordering, path::Path, str::FromStr};

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Integer(u32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct List(Vec<Value>);

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left = self.0.iter();
        let mut right = other.0.iter();

        loop {
            return match (left.next(), right.next()) {
                (Some(left), Some(right)) if left == right => {
                    // in the event the two sides are equal, keep checking
                    // further items in the list
                    continue;
                }
                (Some(left), Some(right)) => left.cmp(right),
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            };
        }
    }
}

#[derive(Debug, Clone, From, PartialEq, Eq)]
enum Value {
    List(List),
    Int(Integer),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
fn list_of(int: &Integer) -> Value {
    List(vec![Value::Int(*int)]).into()
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::{Int, List};

        match (self, other) {
            (Int(left), Int(right)) => left.cmp(right),
            (List(left), List(right)) => left.cmp(right),
            (List(_), Int(right)) => self.cmp(&list_of(right)),
            (Int(left), List(_)) => list_of(left).cmp(other),
        }
    }
}

struct Pair {
    left: List,
    right: List,
}

impl FromStr for Pair {
    type Err = parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::trim).collect::<Vec<_>>();
        if lines.len() < 2 {
            return Err(parse::ParseError::TooShort);
        }
        if lines[2..].iter().any(|line| !line.is_empty()) {
            return Err(parse::ParseError::ExtraTokens);
        }
        Ok(Pair {
            left: List::from_str(lines[0])?,
            right: List::from_str(lines[1])?,
        })
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let ordered_pairs_sum: u32 = parse_newline_sep::<Pair>(input)?
        .enumerate()
        .filter_map(|(idx, Pair { left, right })| {
            (left < right)
                // AoC indices start from 1
                .then_some(idx as u32 + 1)
        })
        .sum();
    println!("sum of indices of ordered pairs: {ordered_pairs_sum}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let divider_two = List::from_str("[[2]]").unwrap();
    let divider_six = List::from_str("[[6]]").unwrap();

    let input = std::fs::read_to_string(input)?;
    let mut packets = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(List::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    packets.push(divider_two.clone());
    packets.push(divider_six.clone());
    packets.sort();

    let two_idx = packets
        .iter()
        .position(|item| *item == divider_two)
        .expect("we have definitely included this divider packet")
        // AoC indices are 1-based
        + 1;
    let six_idx = packets
        .iter()
        .position(|item| *item == divider_six)
        .expect("we have definitely included this divider packet")
        // AoC indices are 1-based
        + 1;

    let decoder_key = two_idx * six_idx;
    println!("decoder key: {decoder_key}");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
    #[error("parsing input")]
    Parse(#[from] parse::ParseError),
}
