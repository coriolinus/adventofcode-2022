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

        // we're going to move the offset left by 1 to ensure there's a blank space for sand to fall
        let min = Point::new(min.x - 1, min.y);

        // note that we add to the width and height to ensure that we capture all relevant information
        let mut map = Map::new_offset(
            min,
            (max.x - min.x) as usize + 3,
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

    /// Drop a single unit of sand. Return whether or not it came to rest.
    pub fn drop_sand(&mut self) -> bool {
        let mut sand = SAND_SOURCE;

        loop {
            let prev_position = sand;

            for deltas in [
                // straight down
                (0, 1),
                // down and to the left
                (-1, 1),
                // down and do the right
                (1, 1),
            ] {
                let dest = sand + deltas;
                if self.map.in_bounds(dest) && !self.map[dest].is_blocked() {
                    sand += deltas;
                    break;
                }
            }

            if sand == prev_position {
                break;
            }
        }

        // we have to add 1 to the sand y position to properly check whether
        // we're in the bottom row. If we are in fact in the bottom row, then
        // we've fallen into the abyss.
        let came_to_rest = sand.y as usize + 1 != self.map.height();
        if came_to_rest {
            self.map[sand] = Tile::Sand;
        }
        came_to_rest
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.flip_vertical().fmt(f)
    }
}
