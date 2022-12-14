use std::{fmt, path::Path};

use aoclib::{
    geometry::{Map, Point},
    parse,
};

use crate::{rock_path::RockPath, tile::Tile, Error, SAND_SOURCE};

/// Find the `(min, max)` boundary points from the input.
fn find_boundaries(points: impl IntoIterator<Item = Point>) -> Option<(Point, Point)> {
    let mut min: Option<Point> = None;
    let mut max: Option<Point> = None;

    for point in points.into_iter() {
        min = min
            .map(|mut min| {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                min
            })
            .or(Some(point));
        max = max
            .map(|mut max| {
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
                max
            })
            .or(Some(point));
    }

    min.zip(max)
}

pub struct Cavern {
    map: Map<Tile>,
}

impl Cavern {
    pub fn parse(input: &Path) -> Result<Self, Error> {
        let paths = parse::<RockPath>(input)?.collect::<Vec<_>>();
        let (min, max) = find_boundaries(
            paths
                .iter()
                .flat_map(|path| path.nodes.iter())
                .copied()
                .chain(std::iter::once(SAND_SOURCE)),
        )
        .ok_or(Error::BadInput("no path nodes"))?;

        // note that we add 1 to the width and height to ensure that we capture all relevant information
        let mut map = Map::new_offset(
            min,
            (max.x - min.x) as usize + 1,
            (max.y - min.y) as usize + 1,
        );

        for path in paths {
            for window in path.nodes.windows(2) {
                let [from, to]: [Point; 2] = window
                    .try_into()
                    .expect("windows always returns slices of length 2");
                if !(from.x == to.x || from.y == to.y) {
                    return Err(Error::BadInput("path segment was not orthogonal"));
                }

                let Point {
                    x: mut dx,
                    y: mut dy,
                } = to - from;
                dx = dx.clamp(-1, 1);
                dy = dy.clamp(-1, 1);

                for point in map
                    .project(from, dx, dy)
                    .take((to - from).manhattan() as usize + 1)
                {
                    map[point] = Tile::Rock;
                }
            }
        }

        Ok(Self { map })
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.flip_vertical().fmt(f)
    }
}
