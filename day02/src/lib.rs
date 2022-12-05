use aoclib::parse;
use parse_display::{Display, FromStr};
use std::path::Path;

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
        if self.me.beats() == self.opponent {
            6
        } else if self.me.loses_against() == self.opponent {
            0
        } else {
            debug_assert_eq!(self.me, self.opponent);
            3
        }
    }

    fn score(self) -> u32 {
        self.outcome_score() + self.me.score()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let total_score: u32 = parse(input)?
        .map(|instruction| {
            let round = Round::from_instruction_pt1(instruction);
            round.score()
        })
        .sum();
    println!("total score (pt. 1): {total_score}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let total_score: u32 = parse(input)?
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
