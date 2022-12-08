use aoclib::geometry::{
    map::{tile::Digit, Map},
    Direction, Point,
};
use std::{collections::HashSet, path::Path};

/// Return the trees which are visible from this projection.
///
/// Visible trees are those whose height is greater than any so far.
///
/// The semantics of this function only work if the projection produces a straight line
/// of points inwards from the edge of the map.
fn filter_visible<'a>(
    map: &'a Map<Digit>,
    projection: impl 'a + Iterator<Item = Point>,
) -> impl 'a + Iterator<Item = Point> {
    let mut highest_yet_seen = None;
    projection.filter(move |point| {
        let point = *point;
        let is_highest = match highest_yet_seen {
            Some(highest) => map[point] > map[highest],
            None => true,
        };
        if is_highest {
            highest_yet_seen = Some(point);
        }
        is_highest
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let trees = <Map<Digit> as TryFrom<&Path>>::try_from(input)?;
    let n_visible = Direction::iter()
        .flat_map(|direction| trees.edge(direction).map(move |point| (direction, point)))
        .flat_map(|(direction, edge_point)| {
            let (dx, dy) = direction.reverse().deltas();
            filter_visible(&trees, trees.project(edge_point, dx, dy))
        })
        .collect::<HashSet<_>>()
        .len();
    println!("n visible trees: {n_visible}");
    Ok(())
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
}
