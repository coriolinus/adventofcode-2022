mod elevation;
mod height_map;
mod path_node;

use aoclib::geometry::Point;
use height_map::HeightMap;
use path_node::PathNode;

use std::{
    collections::{HashSet, VecDeque},
    path::Path,
    rc::Rc,
};

fn find_path(
    map: &HeightMap,
    initial: Point,
    is_goal: impl Fn(&Rc<PathNode>) -> bool,
    can_step: impl Fn(Point, Point) -> bool,
) -> Option<Rc<PathNode>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Rc::new(PathNode {
        location: initial,
        prev: None,
    }));

    while let Some(node) = queue.pop_front() {
        if is_goal(&node) {
            return Some(node);
        }

        if !visited.insert(node.location) {
            // we've already visited this location
            continue;
        }

        queue.extend(
            map.adjacencies(node.location)
                .filter(|&location| {
                    // we can't travel diagonally
                    (location - node.location).manhattan() == 1
                        // external rules about how we can step
                        && can_step(node.location, location)
                        // we don't want to go backwards
                        && !visited.contains(&location)
                })
                .map(|location| {
                    Rc::new(PathNode {
                        location,
                        prev: Some(node.clone()),
                    })
                }),
        )
    }

    None
}

fn solve(
    input: &Path,
    display: &str,
    find_path: impl Fn(&HeightMap) -> Option<Rc<PathNode>>,
) -> Result<(), Error> {
    let map = HeightMap::new(input)?;
    let path_to_destination = find_path(&map).ok_or(Error::NoSolution)?;
    let node_count = path_to_destination.iter().count();
    // we can't parse a map on which the start and target positions are identical, so we know that there
    // are at least two nodes in every correct solution.
    let step_count = node_count - 1;
    println!("{display}: {step_count}");
    Ok(())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    solve(input, "1: steps in shortest path", |map| {
        find_path(
            map,
            map.start,
            |node| node.location == map.target,
            |from, to| map[to] <= map[from] + 1,
        )
    })
}

pub fn part2(input: &Path) -> Result<(), Error> {
    solve(input, "2: steps in shortest possible path", |map| {
        find_path(
            map,
            map.target,
            |node| map[node.location] == 0,
            |from, to| map[to] >= map[from] - 1,
        )
    })
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
    #[error("reading height map")]
    ElevationParse(#[from] height_map::Error),
}
