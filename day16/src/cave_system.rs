use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::{
    state::{HashState, State},
    Action, Error, Flow, Time, ValveReport, START,
};

#[derive(Debug, Default, Clone)]
pub(crate) struct Valve {
    pub(crate) flow_rate: Flow,
    pub(crate) connections: Vec<usize>,
}

type DistanceTable = aoclib::geometry::Map<Time>;
type Memo = HashMap<HashState, Option<Rc<State>>>;

fn compute_distance(valves: &[Valve], from: usize, to: usize) -> Time {
    let mut visited = HashSet::<usize>::new();
    let mut queue = VecDeque::new();

    queue.push_back((0, from));
    while let Some((steps, position)) = queue.pop_front() {
        if position == to {
            return steps;
        } else if !visited.insert(position) {
            // insert returns false when the set already contains the value
            continue;
        } else {
            queue.extend(
                valves[position]
                    .connections
                    .iter()
                    .copied()
                    .filter(|next_valve| !visited.contains(next_valve))
                    .map(|next_valve| (steps + 1, next_valve)),
            );
        }
    }
    !0
}

fn compute_distance_table(valves: &[Valve]) -> DistanceTable {
    let mut map = DistanceTable::new(valves.len(), valves.len());
    for from in 0..valves.len() {
        for to in 0..valves.len() {
            map[(from, to)] = compute_distance(valves, from, to);
        }
    }
    map
}

#[derive(Debug, Default, Clone)]
pub struct CaveSystem {
    pub(crate) valve_names: HashMap<String, usize>,
    pub(crate) valve_indices: Vec<String>,
    pub(crate) valves: Vec<Valve>,
    /// This map stores the traversal distance from any point to any other point.
    ///
    /// Use it like `let steps = self.distance_map[(source, destination)]`.
    pub(crate) distance_table: DistanceTable,
}

impl FromIterator<ValveReport> for Result<CaveSystem, Error> {
    fn from_iter<T: IntoIterator<Item = ValveReport>>(iter: T) -> Self {
        let err = |reason| Err(Error::MalformedCaveSystem { reason });

        let iter = iter.into_iter();
        let reports = {
            let (size_hint, _) = iter.size_hint();
            let mut reports = Vec::with_capacity(size_hint);
            reports.extend(iter);
            reports
        };

        let mut valve_names = HashMap::with_capacity(reports.len());
        let mut valve_indices = Vec::with_capacity(reports.len());
        let mut valves = Vec::with_capacity(reports.len());

        // first pass: initialize valve names and flow rates
        for ValveReport {
            name, flow_rate, ..
        } in reports.iter()
        {
            let idx = valves.len();
            valve_names.insert(name.clone(), idx);
            valve_indices.push(name.clone());
            valves.push(Valve {
                flow_rate: *flow_rate,
                connections: Vec::new(),
            });
        }

        // second pass: initialize connections
        for ValveReport {
            name, downstream, ..
        } in reports.iter()
        {
            let valve_idx = valve_names[name];
            for connection in downstream
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                let conn_idx = valve_names[connection];
                valves[valve_idx].connections.push(conn_idx);
            }
        }

        // third pass: validate connections
        for idx in 0..valves.len() {
            {
                let valve = valves.get_mut(idx).expect("valves list shrank");
                valve.connections.sort_unstable();
                let n_connections = valve.connections.len();
                valve.connections.dedup();
                if valve.connections.len() != n_connections {
                    return err("valve had duplicate connections");
                }
            }

            if !valves[idx]
                .connections
                .iter()
                .copied()
                .all(|conn_idx| valves[conn_idx].connections.contains(&idx))
            {
                return err("all connections are bidirectional");
            }
        }

        if valve_names.len() != valve_indices.len() {
            return err("valve names must not be duplicated");
        }

        let distance_table = compute_distance_table(&valves);

        Ok(CaveSystem {
            valve_names,
            valve_indices,
            valves,
            distance_table,
        })
    }
}

impl CaveSystem {
    fn actions_for<'a>(&'a self, state: &'a State) -> impl 'a + Iterator<Item = Action> {
        use std::iter;

        iter::once(Action::Open {
            valve: state.position,
        })
        .filter(|_| state.valve_states & (1 << state.position) == 0)
        .chain(self.valves.iter().enumerate().filter_map(
            move |(next_valve_idx, next_valve)| {
                (
                    // if we're moving, pick a destination other than self
                    next_valve_idx != state.position
                    // the valve must have a non-0 flow rate
                    && next_valve.flow_rate > 0
                    // the valve must not already be open
                    && state.valve_states & (1 << next_valve_idx) == 0
                    // we must arrive at the destination with at least one time
                    // unit remaining
                    && state.time_to_live > self.distance_table[(state.position, next_valve_idx)]
                )
                    .then_some(Action::Move {
                        from: state.position,
                        to: next_valve_idx,
                    })
            },
        ))
    }

    fn initial_state(&self, time_to_live: Time) -> Result<Rc<State>, Error> {
        let position = *self.valve_names.get(START).ok_or(Error::NoSolution)?;
        Ok(Rc::new(State {
            previous: None,
            position,
            valve_states: 0,
            time_to_live,
        }))
    }

    // Depth-first memoizing solver.
    fn find_max_flow(&self, memo: &mut Memo, given: Rc<State>) -> Option<Rc<State>> {
        eprintln!("find_max_flow(given: {:?})", given.debug(self));
        if given.time_to_live == 0 {
            return Some(given);
        }
        let memo_key: HashState = (&*given).into();

        if let Some(memoized) = memo.get(&memo_key) {
            return memoized.clone();
        }

        let best_successor = {
            let mut found = None;
            for action in self.actions_for(&given) {
                dbg!(action);
                let Some(successor) = given.apply(action, self) else {
                    continue;
                };
                dbg!(successor.debug(self));

                eprintln!("RECUR find_max_flow(successor)");
                let max_flow = self.find_max_flow(memo, successor);
                eprintln!("END RECUR found: {}", max_flow.is_some());
                let Some(max_flow) = max_flow else {
                    continue;
                };

                let flow_rate = max_flow.total_flow(self);
                found = match found.take() {
                    Some((old_max_rate, succ)) => {
                        if flow_rate > old_max_rate {
                            Some((flow_rate, max_flow))
                        } else {
                            Some((old_max_rate, succ))
                        }
                    }
                    None => Some((flow_rate, max_flow)),
                };
            }
            found.map(|(_rate, succ)| succ)
        };

        dbg!(&best_successor.as_ref().map(|succ| succ.debug(self)));

        memo.insert(memo_key, best_successor.clone());
        best_successor
    }

    // /// Breadth-first search through sequence options.
    // fn consider_sequences(&self, mut consider: impl FnMut(Rc<State>)) -> Result<(), Error> {
    //     let mut queue = VecDeque::new();
    //     queue.push_back(self.initial_state()?);

    //     while let Some(state) = queue.pop_front() {
    //         #[cfg(debug_assertions)]
    //         if state.time > TIME_TO_LIVE {
    //             panic!("depth exceeded TTL")
    //         }
    //         if state.time == TIME_TO_LIVE {
    //             consider(state);
    //         } else {
    //             queue.extend(
    //                 self.actions_for(&state)
    //                     .filter_map(|action| state.apply(action, self)),
    //             );
    //         }
    //     }

    //     Ok(())
    // }

    // pub(crate) fn find_max_total_flow(&self) -> Result<Rc<State>, Error> {
    //     let mut max: Option<Rc<State>> = None;
    //     self.consider_sequences(|state| {
    //         max = Some(match max.take() {
    //             Some(max) => {
    //                 if state.total_flow(self) > max.total_flow(self) {
    //                     state
    //                 } else {
    //                     max
    //                 }
    //             }
    //             None => state,
    //         });
    //         dbg!(
    //             max.as_ref().map(|state| state.debug(self)),
    //             max.as_ref().map(|state| state.total_flow(self)),
    //         );
    //     })?;
    //     max.ok_or(Error::NoSolution)
    // }

    pub(crate) fn find_max_total_flow(&self, time_to_live: Time) -> Result<Rc<State>, Error> {
        let mut memo = Memo::new();
        let initial = self.initial_state(time_to_live)?;
        self.find_max_flow(&mut memo, initial)
            .ok_or(Error::NoSolution)
    }
}
