use std::str::FromStr;

use aoclib::geometry::tile::DisplayWidth;

#[derive(Debug, Clone, Copy)]
pub enum Elevation {
    Height(u8),
    Start,
    Target,
}

impl FromStr for Elevation {
    type Err = ElevationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ElevationParseError::WrongWidth(s.len()));
        }
        let b = s.as_bytes();
        match b[0] {
            b'S' => Ok(Elevation::Start),
            b'E' => Ok(Elevation::Target),
            _ if (b'a'..=b'z').contains(&b[0]) => Ok(Elevation::Height(b[0] - b'a')),
            _ => Err(ElevationParseError::OutOfBounds),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ElevationParseError {
    #[error("wrong input width: have {0}; expected 1")]
    WrongWidth(usize),
    #[error("elevation must be in 'a'..='z'")]
    OutOfBounds,
}

impl DisplayWidth for Elevation {
    const DISPLAY_WIDTH: usize = 1;
}
