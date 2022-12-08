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

/// Return those coordinates which are on the map and visible from the specified projection.
fn visible_from(
    map: &Map<Digit>,
    origin: Point,
    direction: Direction,
) -> impl '_ + Iterator<Item = Point> {
    let mut found_blocker = false;
    let (dx, dy) = direction.deltas();
    let initial_height = map[origin];
    map.project(origin, dx, dy)
        // origin is the first point in the projection
        .skip(1)
        .take_while(move |point| {
            if found_blocker {
                false
            } else {
                found_blocker |= map[*point] >= initial_height;
                true
            }
        })
}

fn scenic_score(map: &Map<Digit>, origin: Point) -> usize {
    Direction::iter()
        .map(|direction| visible_from(map, origin, direction).count())
        .product()
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
    // We can't re-use the result from part 1 to filter the points to consider here.
    // Consider a map whose perimeter trees all have height 9. They all have a scenic
    // score of 0, because there is at least one direction in which they can see no
    // other trees at all. However, they block all potential inner trees which might have a
    // higher score.

    let trees = <Map<Digit> as TryFrom<&Path>>::try_from(input)?;
    let Some(max_scenic_score) = trees.points().map(|point| scenic_score(&trees, point)).max() else {
        println!("map has size 0");
        return Ok(())
    };

    println!("max scenic score: {max_scenic_score}");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
