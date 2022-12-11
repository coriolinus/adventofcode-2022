use std::{
    collections::VecDeque,
    ops::{Add, Mul},
};

use derive_builder::Builder;
use parse_display::{Display, FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromStr, Display)]
pub struct MonkeyId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
pub enum Arithmetic {
    #[display("*")]
    Multiply,
    #[display("+")]
    Add,
}

impl Arithmetic {
    fn apply<T>(self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Arithmetic::Multiply => a * b,
            Arithmetic::Add => a + b,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
#[display(style = "lowercase")]
pub enum Operand {
    Old,
    #[display("{0}")]
    Value(u128),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
#[display("Operation: new = old {arithmetic} {operand}")]
pub struct Operation {
    pub arithmetic: Arithmetic,
    pub operand: Operand,
}

impl Operation {
    pub fn perform(self, value: u128) -> u128 {
        let operand_value = match self.operand {
            Operand::Old => value,
            Operand::Value(v) => v,
        };
        self.arithmetic.apply(value, operand_value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
#[display("Test: divisible by {divisible_by}")]
pub struct Test {
    pub divisible_by: u128,
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Monkey {
    pub id: MonkeyId,
    pub items: VecDeque<u128>,
    pub operation: Operation,
    pub test: Test,
    pub true_destination: MonkeyId,
    pub false_destination: MonkeyId,
    #[builder(setter(skip))]
    pub inspect_count: u32,
}
