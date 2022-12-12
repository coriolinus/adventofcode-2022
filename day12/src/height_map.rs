use std::{
    ops::{Deref, Index},
    path::Path,
};

use aoclib::geometry::{Map, Point};

use crate::elevation::{Elevation, ElevationParseError};

pub struct HeightMap {
    map: Map<u8>,
    pub start: Point,
    pub target: Point,
}

impl HeightMap {
    pub fn new(input: &Path) -> Result<Self, Error> {
        let elevation_map = <Map<Elevation> as TryFrom<&Path>>::try_from(input)?;

        let mut map = Map::<u8>::new(elevation_map.width(), elevation_map.height());
        let mut start = None;
        let mut target = None;

        for (location, tile) in elevation_map.iter() {
            let height = match tile {
                Elevation::Height(height) => *height,
                Elevation::Start => match start {
                    Some(_) => return Err(Error::WrongNumberStartOrTarget),
                    None => {
                        start = Some(location);
                        0
                    }
                },
                Elevation::Target => match target {
                    Some(_) => return Err(Error::WrongNumberStartOrTarget),
                    None => {
                        target = Some(location);
                        b'z' - b'a'
                    }
                },
            };
            map[location] = height;
        }

        let start = start.ok_or(Error::WrongNumberStartOrTarget)?;
        let target = target.ok_or(Error::WrongNumberStartOrTarget)?;

        Ok(Self { map, start, target })
    }
}

impl Deref for HeightMap {
    type Target = Map<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Index<Point> for HeightMap {
    type Output = u8;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        self.map.index(index)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parsing tile")]
    TileParse(#[from] ElevationParseError),
    #[error("wrong number of start or target tiles; must be 1 each")]
    WrongNumberStartOrTarget,
    #[error("reading input")]
    Io(#[from] std::io::Error),
}
