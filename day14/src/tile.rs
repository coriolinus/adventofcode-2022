use aoclib::geometry::tile::{DisplayWidth, ToRgb};
use parse_display::{Display, FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromStr, Display)]
pub enum Tile {
    #[default]
    #[display(".")]
    Air,
    #[display("o")]
    Sand,
    #[display("#")]
    Rock,
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

impl ToRgb for Tile {
    fn to_rgb(&self) -> [u8; 3] {
        match self {
            Tile::Air => [0, 0, 0],
            Tile::Sand => [211, 214, 171],
            Tile::Rock => [150, 150, 150],
        }
    }
}
