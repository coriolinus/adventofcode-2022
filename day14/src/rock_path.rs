use std::str::FromStr;

use aoclib::geometry::Point;

use crate::Error;

pub struct RockPath {
    pub nodes: Vec<Point>,
}

impl FromStr for RockPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .split("->")
            .map(|point_s| {
                let Some((left, right)) = point_s.split_once(',') else {
                return Err(Error::BadInput("failed to split by comma"));
            };
                let left = left
                    .trim()
                    .parse()
                    .map_err(|_| Error::BadInput("parsing left"))?;
                let right = right
                    .trim()
                    .parse()
                    .map_err(|_| Error::BadInput("parsing left"))?;
                Ok(Point::new(left, right))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { nodes })
    }
}
