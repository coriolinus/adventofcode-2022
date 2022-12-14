mod cavern;
mod rock_path;
mod tile;

use aoclib::geometry::Point;
use cavern::Cavern;
use std::path::Path;

pub const SAND_SOURCE: Point = Point::new(500, 0);

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut cavern = Cavern::parse(input)?;

    let mut came_to_rest = 0;
    while cavern.drop_sand() {
        came_to_rest += 1;
    }

    println!("{came_to_rest} units of sand came to rest");

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
    #[error("bad input: {0}")]
    BadInput(&'static str),
}
