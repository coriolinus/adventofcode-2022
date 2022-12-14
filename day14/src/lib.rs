mod cavern;
mod hooks;
mod rock_path;
mod tile;

use aoclib::geometry::{map::RenderError, Point};
use cavern::Cavern;
use std::path::Path;

pub const SAND_SOURCE: Point = Point::new(500, 0);

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut cavern = Cavern::parse(input)?;

    hooks::pre(&cavern)?;

    let mut came_to_rest = 0;
    while cavern.drop_sand() {
        came_to_rest += 1;
        hooks::trace(&cavern)?;
    }

    hooks::post(&cavern)?;
    println!("{came_to_rest} units of sand came to rest");

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut cavern = Cavern::parse(input)?;
    cavern.add_floor();

    hooks::pre(&cavern)?;

    let mut dropped_units = 0;
    while !cavern.map[SAND_SOURCE].is_blocked() {
        let _came_to_rest = cavern.drop_sand();
        debug_assert!(_came_to_rest, "sand must have stopped");
        dropped_units += 1;
        hooks::trace(&cavern)?;
    }

    hooks::post(&cavern)?;
    println!("{dropped_units} units of sand came to rest");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
    #[error("bad input: {0}")]
    BadInput(&'static str),
    #[error("rendering image")]
    Rendering(#[from] RenderError),
    #[error("adding frame to animation")]
    Animating(#[from] aoclib::geometry::map::EncodingError),
}
