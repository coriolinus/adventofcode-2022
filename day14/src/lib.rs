mod cavern;
mod rock_path;
mod tile;

use aoclib::geometry::Point;
use cavern::Cavern;
use std::path::Path;

pub const SAND_SOURCE: Point = Point::new(500, 0);

fn pre_hook(cavern: &Cavern) -> Result<(), Error> {
    use std::env::var;

    if var("CONSOLE_PRE")
        .map(|val| !val.is_empty())
        .unwrap_or_default()
    {
        println!("{cavern}");
    }

    // if let Ok(path) = var("IMAGE_PRE") {
    //     cavern.map.render(Path::new(&path), Style::Grid)?;
    // }

    Ok(())
}

fn post_hook(cavern: &Cavern) -> Result<(), Error> {
    use std::env::var;

    if var("CONSOLE_POST")
        .map(|val| !val.is_empty())
        .unwrap_or_default()
    {
        println!("{cavern}");
    }

    // if let Ok(path) = var("IMAGE_POST") {
    //     cavern.map.render(Path::new(&path), Style::Grid)?;
    // }

    Ok(())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut cavern = Cavern::parse(input)?;

    pre_hook(&cavern)?;

    let mut came_to_rest = 0;
    while cavern.drop_sand() {
        came_to_rest += 1;
    }

    post_hook(&cavern)?;
    println!("{came_to_rest} units of sand came to rest");

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut cavern = Cavern::parse(input)?;
    cavern.add_floor();

    pre_hook(&cavern)?;

    let mut dropped_units = 0;
    while !cavern.map[SAND_SOURCE].is_blocked() {
        let _came_to_rest = cavern.drop_sand();
        debug_assert!(_came_to_rest, "sand must have stopped");
        dropped_units += 1;
    }

    post_hook(&cavern)?;
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
}
