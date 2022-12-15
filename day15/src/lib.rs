mod range;

use aoclib::{geometry::Point, parse};
use parse_display::{Display, FromStr};
use std::{ops::RangeInclusive, path::Path};

use crate::range::{contained_points, merge_ranges};

#[derive(Default, Clone, Copy, Display, FromStr)]
#[display("Sensor at x={sensor.x}, y={sensor.y}: closest beacon is at x={beacon.x}, y={beacon.y}")]
#[from_str(default)]
struct Report {
    sensor: Point,
    beacon: Point,
}

impl Report {
    fn impossible_positions_at_row(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let distance = (self.beacon - self.sensor).manhattan();
        let y_component = (y - self.sensor.y).abs();
        let x_component = distance - y_component;
        let min_x = self.sensor.x - x_component;
        let max_x = self.sensor.x + x_component;

        (min_x <= max_x).then(|| {
            debug_assert_eq!(
                distance,
                (Point::new(min_x, y) - self.sensor).manhattan(),
                "identifying min x at row"
            );
            debug_assert_eq!(
                distance,
                (Point::new(max_x, y) - self.sensor).manhattan(),
                "identifying max x at row"
            );
            min_x..=max_x
        })
    }
}

// wrong: too high: 4748136
//                  4748136
pub fn part1(input: &Path) -> Result<(), Error> {
    const ROW: i32 = 2_000_000;

    let impossible_count: u64 = merge_ranges(
        parse::<Report>(input)?.filter_map(|report| report.impossible_positions_at_row(ROW)),
    )
    .into_iter()
    .map(contained_points)
    .sum();

    println!("{impossible_count} impossible positions at y={ROW}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_report() {
        let report = Report {
            sensor: Point::new(8, 7),
            beacon: Point::new(2, 10),
        };

        eprintln!("bottom half:");
        for (idx, y) in (-3..=7).enumerate() {
            let Some(range) = report.impossible_positions_at_row(y) else {
                continue;
            };
            let (low, high) = range.into_inner();
            let contained_points = if low <= high { high - low + 1 } else { 0 };
            dbg!(y, low, high, contained_points, idx);
            let expect = (2 * idx).checked_sub(1).unwrap_or_default();
            assert_eq!(contained_points as usize, expect);
        }
        eprintln!("top half:");
        for (idx, y) in (7..=17).rev().enumerate() {
            let Some(range) = report.impossible_positions_at_row(y) else {
                continue;
            };
            let (low, high) = range.into_inner();
            let contained_points = if low <= high { high - low + 1 } else { 0 };
            dbg!(low, high, contained_points, idx);
            let expect = (2 * idx).checked_sub(1).unwrap_or_default();
            assert_eq!(contained_points as usize, expect);
        }
    }
}
