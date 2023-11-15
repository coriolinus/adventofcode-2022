mod cave_system;
mod state;

use aoclib::parse;
use enum_as_inner::EnumAsInner;
use parse_display::{Display, FromStr};
use std::path::Path;

use crate::cave_system::CaveSystem;

type Time = u8;
type Flow = u32;
// we know there are only 54 valves in the problem input, so this is sufficient
// for a more general solution we'd probably want a more extensive bitflags implementation
type ValveStates = u64;

const START: &str = "AA";
const TIME_TO_LIVE: Time = 30;

#[derive(Debug, FromStr, Display)]
#[display("Valve {name} has flow rate={flow_rate}; tunnels lead to valves {downstream}")]
#[from_str(
    regex = r"^Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves?(?P<downstream>[\w, ]*)$"
)]
struct ValveReport {
    name: String,
    flow_rate: Flow,
    downstream: String,
}

#[derive(Debug, Clone, Copy, EnumAsInner)]
pub(crate) enum Action {
    Move { from: usize, to: usize },
    Open { valve: usize },
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let cave_system = parse::<ValveReport>(input)?.collect::<Result<CaveSystem, _>>()?;
    let state = cave_system.find_max_total_flow(TIME_TO_LIVE)?;
    debug_assert_eq!(state.time_to_live, TIME_TO_LIVE);
    state.report(&cave_system, TIME_TO_LIVE);
    let flow = state.total_flow(&cave_system);
    println!("max total flow: {flow}");
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
    #[error("malformed cave system: {reason}")]
    MalformedCaveSystem { reason: &'static str },
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use aoclib::input::parse_str;

    use crate::state::State;

    use super::*;

    fn run(definition: &str, time: Time) -> (CaveSystem, Rc<State>) {
        let cave_system = parse_str(definition)
            .unwrap()
            .collect::<Result<CaveSystem, _>>()
            .unwrap();

        let state = cave_system.find_max_total_flow(time).unwrap();
        state.report(&cave_system, time);
        (cave_system, state)
    }

    #[test]
    fn test_0() {
        let definition = "Valve AA has flow rate=0; tunnels lead to valves";
        let (cave_system, state) = run(definition, 0);
        assert_eq!(state.total_flow(&cave_system), 0);
    }

    #[test]
    fn test_turn_on() {
        let definition = "Valve AA has flow rate=1; tunnels lead to valves";
        let (cave_system, state) = run(definition, 1);
        assert_eq!(state.total_flow(&cave_system), 0);
    }

    #[test]
    fn test_turn_on_and_move() {
        let definition = "
Valve AA has flow rate=1; tunnels lead to valves BB
Valve BB has flow rate=1; tunnels lead to valves AA
        "
        .trim();
        let (cave_system, state) = run(definition, 2);
        assert_eq!(state.position, cave_system.valve_names["BB"]);
        assert_eq!(state.total_flow(&cave_system), 1);
    }
}
