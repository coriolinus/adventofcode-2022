use aoclib::parse;
use parse_display::{Display, FromStr};
use std::path::Path;

const START: &'static str = "AA";

#[derive(Debug, FromStr, Display)]
#[display("Valve {name} has flow rate={flow_rate}; tunnels lead to valves {downstream}")]
#[from_str(
    regex = r"^Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<downstream>[\w, ]+)$"
)]
struct ValveReport {
    name: String,
    flow_rate: u32,
    downstream: String,
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let n_valves = parse::<ValveReport>(input)?.count();
    println!("parsed {n_valves} valves");
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
