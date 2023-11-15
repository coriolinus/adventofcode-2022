use std::rc::Rc;

use crate::{
    cave_system::{CaveSystem, Valve},
    Action, Flow, Time, ValveStates,
};

pub struct State {
    pub(crate) previous: Option<Rc<State>>,
    pub(crate) position: usize,
    pub(crate) valve_states: ValveStates,
    pub(crate) time_to_live: Time,
}

impl State {
    fn open_valve(self: &Rc<Self>) -> Option<Rc<Self>> {
        let my_state = 1 << self.position;
        let time_to_live = self.time_to_live.checked_sub(1)?;
        (self.valve_states & my_state == 0).then(|| {
            Rc::new(State {
                previous: Some(self.clone()),
                position: self.position,
                valve_states: self.valve_states | my_state,
                time_to_live,
            })
        })
    }

    fn move_to(self: &Rc<Self>, to: usize, cave_system: &CaveSystem) -> Option<Rc<Self>> {
        let time_to_live = self
            .time_to_live
            .checked_sub(cave_system.distance_table[(self.position, to)])?;
        Some(Rc::new(State {
            previous: Some(self.clone()),
            position: to,
            valve_states: self.valve_states,
            time_to_live,
        }))
    }

    pub(crate) fn apply(
        self: &Rc<Self>,
        action: Action,
        cave_system: &CaveSystem,
    ) -> Option<Rc<Self>> {
        match action {
            Action::Move { from, to } => {
                debug_assert_eq!(from, self.position, "wrong from in this move action");
                self.move_to(to, cave_system)
            }
            Action::Open { valve } => {
                debug_assert_eq!(valve, self.position, "wrong valve in this open action");
                self.open_valve()
            }
        }
    }

    fn open_valves<'a>(
        &'a self,
        cave_system: &'a CaveSystem,
    ) -> impl '_ + Iterator<Item = (usize, &'a Valve)> {
        cave_system
            .valves
            .iter()
            .enumerate()
            .filter(|(idx, _valve)| self.valve_states & (1 << idx) != 0)
    }

    fn flow_rate(&self, cave_system: &CaveSystem) -> Flow {
        self.open_valves(cave_system)
            .map(|(_idx, valve)| valve.flow_rate)
            .sum()
    }

    fn total_flow_inner(&self, cave_system: &CaveSystem) -> Flow {
        self.previous
            .as_ref()
            .map(|prev| prev.total_flow_inner(cave_system))
            .unwrap_or_default()
            + self.flow_rate(cave_system)
    }

    pub fn total_flow(&self, cave_system: &CaveSystem) -> Flow {
        self.previous
            .as_ref()
            .map(|prev| prev.total_flow_inner(cave_system))
            .unwrap_or_default()
    }

    // we may not be calling it right now, but we want the ability to call this
    // and get a debug struct with nice formatting
    #[allow(dead_code)]
    pub fn debug<'a>(&self, cave_system: &'a CaveSystem) -> Debug<'a> {
        Debug {
            time_to_live: self.time_to_live,
            position: &cave_system.valve_indices[self.position],
            valve_states: format!("{:064b}", self.valve_states),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn hash(&self) -> HashState {
        self.into()
    }

    pub fn report(&self, cave_system: &CaveSystem, initial_time: Time) {
        if let Some(previous) = &self.previous {
            previous.report(cave_system, initial_time);
            println!();
        }

        println!("== Minute {} ==", initial_time - self.time_to_live);
        println!("Position: {}", &cave_system.valve_indices[self.position]);
        let open_valves = self
            .open_valves(cave_system)
            .map(|(idx, _)| cave_system.valve_indices[idx].clone())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Open Valves: {open_valves}");
        println!("Flow rate:  {}", self.flow_rate(cave_system));
        println!("Total flow: {}", self.total_flow(cave_system));
    }
}

#[derive(Debug)]
pub struct Debug<'a> {
    pub time_to_live: Time,
    pub position: &'a str,
    pub valve_states: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct HashState {
    pub time_to_live: Time,
    pub position: usize,
    pub valve_states: ValveStates,
}

impl From<&State> for HashState {
    fn from(value: &State) -> Self {
        Self {
            time_to_live: value.time_to_live,
            position: value.position,
            valve_states: value.valve_states,
        }
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        HashState::from(self).hash(state)
    }
}
