mod range;

use aoclib::{geometry::Point, parse};
use parse_display::{Display, FromStr};
use std::{collections::HashSet, ops::RangeInclusive, path::Path};

use crate::range::{contained_points, find_excluded, merge_ranges};

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

pub fn part1(input: &Path) -> Result<(), Error> {
    const ROW: i32 = 2_000_000;
    let row = std::env::var("ROW")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(ROW);

    let reports = parse::<Report>(input)?.collect::<Vec<_>>();

    let range_count: u64 = merge_ranges(
        reports
            .iter()
            .filter_map(|report| report.impossible_positions_at_row(row)),
    )
    .into_iter()
    .map(contained_points)
    .sum();

    let sensors = reports
        .iter()
        .filter_map(|report| (report.sensor.y == row).then_some(report.sensor.x))
        .collect::<HashSet<_>>();
    let sensor_count = sensors.len() as u64;

    let beacons = reports
        .iter()
        .filter_map(|report| (report.beacon.y == row).then_some(report.beacon.x))
        .collect::<HashSet<_>>();
    let beacon_count = beacons.len() as u64;

    let impossible_count = range_count - sensor_count - beacon_count;

    println!("{impossible_count} impossible positions at y={row}");
    println!("  (range {range_count} - {sensor_count} sensors - {beacon_count} beacons)");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    const UPPER_BOUND: i32 = 4_000_000;

    let upper_bound = std::env::var("UPPER_BOUND")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(UPPER_BOUND);

    let bounds = 0..=upper_bound;

    let reports = parse::<Report>(input)?.collect::<Vec<_>>();

    let (x, y) = bounds
        .clone()
        .find_map(|row| {
            let excluded = merge_ranges(
                reports
                    .iter()
                    .filter_map(|report| report.impossible_positions_at_row(row)),
            );
            find_excluded(&bounds, &excluded).map(|column| (column as u64, row as u64))
        })
        .ok_or(Error::NoSolution)?;

    let tuning_frequency = x * 4000000 + y;
    println!("tuning frequency: {tuning_frequency}");
    println!("  at ({x}, {y})");

    Ok(())
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
