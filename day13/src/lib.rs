use aoclib::input::parse_newline_sep;
use derive_more::From;
use std::{path::Path, str::FromStr};

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Integer(u32);

#[derive(Debug, Clone)]
struct List(Vec<Value>);

impl List {
    fn iter(&self) -> impl '_ + Iterator<Item = &Value> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, From)]
enum Value {
    List(List),
    Int(Integer),
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

#[inline]
fn list_of(int: &Integer) -> List {
    List(vec![Value::Int(*int)])
}

fn is_ordered(left: &Value, right: &Value) -> Option<bool> {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use Value::{Int, List};

    match (left, right) {
        (Int(left), Int(right)) => match left.cmp(right) {
            Less => Some(true),
            Equal => None,
            Greater => Some(false),
        },
        (List(left), List(right)) => compare_lists(left, right),
        (List(_), Int(right)) => is_ordered(left, &list_of(right).into()),
        (Int(left), List(_)) => is_ordered(&list_of(left).into(), right),
    }
}

fn compare_lists(left: &List, right: &List) -> Option<bool> {
    let mut left = left.iter();
    let mut right = right.iter();

    loop {
        match (left.next(), right.next()) {
            (Some(left), Some(right)) => {
                let potential_order = is_ordered(left, right);
                if potential_order.is_some() {
                    return potential_order;
                }
            }
            (None, Some(_)) => return Some(true),
            (Some(_), None) => return Some(false),
            (None, None) => return None,
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let ordered_pairs_sum: u32 = parse_newline_sep::<Pair>(input)?
        .enumerate()
        .filter_map(|(idx, Pair { left, right })| {
            compare_lists(&left, &right)
                .unwrap_or_default()
                // AoC indices start from 1
                .then_some(idx as u32 + 1)
        })
        .sum();
    println!("sum of indices of ordered pairs: {ordered_pairs_sum}");
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
}
