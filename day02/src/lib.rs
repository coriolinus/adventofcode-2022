use aoclib::parse;
use parse_display::{Display, FromStr};
use std::{cmp::Ordering, path::Path};

/// Input column 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
enum OpponentSigil {
    A,
    B,
    C,
}

/// Input column 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
enum OtherSigil {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
#[display("{opponent} {other}")]
struct Instruction {
    opponent: OpponentSigil,
    other: OtherSigil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

// Crimes! Rps does not have a total order; it is not transitive.
// Don't try to sort a `Vec<Rps>`!
impl Ord for Rps {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.beats() == *other {
            Ordering::Greater
        } else {
            debug_assert_eq!(self.loses_against(), *other);
            Ordering::Less
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<OpponentSigil> for Rps {
    fn from(sigil: OpponentSigil) -> Self {
        match sigil {
            OpponentSigil::A => Rps::Rock,
            OpponentSigil::B => Rps::Paper,
            OpponentSigil::C => Rps::Scissors,
        }
    }
}

impl From<OtherSigil> for Rps {
    fn from(sigil: OtherSigil) -> Self {
        match sigil {
            OtherSigil::X => Rps::Rock,
            OtherSigil::Y => Rps::Paper,
            OtherSigil::Z => Rps::Scissors,
        }
    }
}

impl Rps {
    const fn score(self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    /// Define the ordering: what does this position beat?
    const fn beats(self) -> Self {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    // There doesn't seem to be an elegant way to derive this, unfortunately.
    const fn loses_against(self) -> Self {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round {
    opponent: Rps,
    me: Rps,
}

impl Round {
    fn from_instruction_pt1(Instruction { opponent, other }: Instruction) -> Self {
        let opponent = opponent.into();
        let me = other.into();
        Self { opponent, me }
    }

    fn from_instruction_pt2(Instruction { opponent, other }: Instruction) -> Self {
        let opponent: Rps = opponent.into();
        let me = match other {
            OtherSigil::X => {
                // we need to lose
                opponent.beats()
            }
            OtherSigil::Y => {
                // we need to draw
                opponent
            }
            OtherSigil::Z => {
                // we need to win
                opponent.loses_against()
            }
        };
        Self { opponent, me }
    }

    fn outcome_score(self) -> u32 {
        match self.me.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }

    fn score(self) -> u32 {
        self.outcome_score() + self.me.score()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let instructions: Vec<Instruction> = parse(input)?.collect();
    let total_score: u32 = instructions
        .into_iter()
        .map(|instruction| {
            let round = Round::from_instruction_pt1(instruction);
            round.score()
        })
        .sum();
    println!("total score (pt. 1): {total_score}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let instructions: Vec<Instruction> = parse(input)?.collect();
    let total_score: u32 = instructions
        .into_iter()
        .map(|instruction| {
            let round = Round::from_instruction_pt2(instruction);
            round.score()
        })
        .sum();
    println!("total score (pt. 2): {total_score}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
