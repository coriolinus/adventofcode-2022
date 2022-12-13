use std::{num::ParseIntError, str::FromStr};

use crate::{Integer, List, Value};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected end of input")]
    TooShort,
    #[error("unexpected token")]
    UnexpectedToken,
    #[error("parsing integer")]
    ParseInt(#[from] ParseIntError),
    #[error("extra tokens")]
    ExtraTokens,
}

impl Integer {
    fn parse(s: &str) -> Result<(Self, &str), ParseError> {
        if s.is_empty() {
            return Err(ParseError::TooShort);
        }
        let split_point = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
        if split_point == 0 {
            return Err(ParseError::UnexpectedToken);
        }
        let (used, rest) = s.split_at(split_point);
        let value = used.parse()?;
        Ok((Integer(value), rest))
    }
}

impl List {
    fn parse(mut s: &str) -> Result<(Self, &str), ParseError> {
        if s.is_empty() {
            return Err(ParseError::TooShort);
        }
        if !s.starts_with('[') {
            return Err(ParseError::UnexpectedToken);
        }
        s = &s[1..];

        let mut inner = Vec::new();
        if let Some(rest) = s.strip_prefix(']') {
            return Ok((Self(inner), rest));
        }

        loop {
            let (value, rest) = Value::parse(s)?;
            inner.push(value);

            if let Some(rest) = rest.strip_prefix(']') {
                return Ok((Self(inner), rest));
            } else if let Some(rest) = rest.strip_prefix(',') {
                s = rest;
            } else {
                return Err(ParseError::UnexpectedToken);
            }
        }
    }
}

impl FromStr for List {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (list, rest) = List::parse(s)?;
        if !rest.is_empty() {
            return Err(ParseError::ExtraTokens);
        }
        Ok(list)
    }
}

impl Value {
    fn parse(s: &str) -> Result<(Self, &str), ParseError> {
        Integer::parse(s)
            .map(|(i, rest)| (i.into(), rest))
            .or_else(|_| List::parse(s).map(|(l, rest)| (l.into(), rest)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,1,3,1,1]")]
    #[case("[1,1,5,1,1]")]
    #[case("[[1],[2,3,4]]")]
    #[case("[[1],4]")]
    #[case("[9]")]
    #[case("[[8,7,6]]")]
    #[case("[[4,4],4,4]")]
    #[case("[[4,4],4,4,4]")]
    #[case("[7,7,7,7]")]
    #[case("[7,7,7]")]
    #[case("[]")]
    #[case("[3]")]
    #[case("[[[]]]")]
    #[case("[[]]")]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]")]
    #[case("[1,[2,[3,[4,[5,6,0]]]],8,9]")]
    fn list_from_str(#[case] input: &str) {
        dbg!(input);
        List::from_str(input).unwrap();
    }
}
