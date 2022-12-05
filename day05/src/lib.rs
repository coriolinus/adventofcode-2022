use parse_display::Display;
use std::{path::Path, str::FromStr};

#[derive(Debug, Clone, Copy, parse_display::FromStr, Display)]
#[display("move {qty} from {origin} to {destination}")]
struct Movement {
    qty: u32,
    origin: u32,
    destination: u32,
}

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<u8>>);

impl FromStr for Stacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev().filter(|line| !line.trim().is_empty());
        let indices = lines
            .next()
            .ok_or(Error::MalformedStacks("no index line found"))?;
        if indices
            .split_whitespace()
            .any(|idx| idx.parse::<usize>().is_err())
        {
            return Err(Error::MalformedStacks("parsing indices as numbers"));
        }
        let n_stacks: usize = indices
            .split_whitespace()
            .last()
            .ok_or(Error::MalformedStacks("no index numbers found"))?
            .parse()
            .map_err(|_| Error::MalformedStacks("parsing final index as usize"))?;

        let mut stacks = vec![Vec::new(); n_stacks];

        for line in lines {
            for (idx, chunk) in line.split_whitespace().enumerate() {
                let chunk = chunk.as_bytes();
                if chunk[0] != b'[' || chunk[2] != b']' {
                    return Err(Error::MalformedStacks("did not find crate edges"));
                }
                stacks[idx].push(chunk[1]);
            }
        }

        Ok(Stacks(stacks))
    }
}

fn parse(input: &Path) -> Result<(Stacks, Vec<Movement>), Error> {
    use aoclib::input::{parse_str, parse_two_phase};

    struct MovementParser(Vec<Movement>);

    impl FromStr for MovementParser {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(MovementParser(parse_str(s)?.collect()))
        }
    }

    parse_two_phase::<'_, Stacks, MovementParser>(input)
        .map(|(stacks, movement_sets)| {
            (
                stacks,
                movement_sets.flat_map(|mp| mp.0.into_iter()).collect(),
            )
        })
        .map_err(Into::into)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let (stacks, movements) = parse(input)?;
    dbg!(stacks.0.len(), movements.len());
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
    #[error("malformed stacks: {0}")]
    MalformedStacks(&'static str),
    #[error(transparent)]
    TwoPhase(#[from] aoclib::input::TwoPhaseError),
}
