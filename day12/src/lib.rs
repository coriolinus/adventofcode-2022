mod elevation;
mod height_map;
mod path_node;

use height_map::HeightMap;
use path_node::PathNode;

use std::{
    collections::{HashSet, VecDeque},
    path::Path,
    rc::Rc,
};

fn find_path_to_destination(map: &HeightMap) -> Option<Rc<PathNode>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Rc::new(PathNode {
        location: map.start,
        prev: None,
    }));

    while let Some(node) = queue.pop_front() {
        if node.location == map.target {
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
                        // we can't climb more than one step at a time
                        && map[location] <= map[node.location] + 1
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

// wrong, too low: 374
pub fn part1(input: &Path) -> Result<(), Error> {
    let map = HeightMap::new(input)?;
    let path_to_destination = find_path_to_destination(&map).ok_or(Error::NoSolution)?;
    let node_count = path_to_destination.iter().count();
    // we can't parse a map on which the start and target positions are identical, so we know that there
    // are at least two nodes in every correct solution.
    let step_count = node_count - 1;
    println!("steps in shortest path: {step_count}");
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
    #[error("reading height map")]
    ElevationParse(#[from] height_map::Error),
}
