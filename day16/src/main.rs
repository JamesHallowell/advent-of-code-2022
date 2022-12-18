use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

mod parsing;

type Valve<'a> = &'a str;

type Minutes = i32;

#[derive(Debug, Clone)]
struct Input<'a> {
    valve: Valve<'a>,
    flow_rate: u64,
    tunnels: Vec<Valve<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node<'a> {
    valve: Valve<'a>,
    flow_rate: u64,
    tunnels: HashMap<Valve<'a>, Minutes>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Searcher<'input> {
    AtValve(Valve<'input>),
    InTunnel {
        destination: Valve<'input>,
        time_until_arrival: Minutes,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State<'input, 'global> {
    me: Searcher<'input>,
    elephant: Searcher<'input>,
    time_left: Minutes,
    flow: u64,
    released_valves: HashSet<Valve<'input>>,
    nodes: &'global HashMap<Valve<'input>, Node<'input>>,
}

impl PartialOrd for State<'_, '_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for State<'_, '_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flow.cmp(&other.flow)
    }
}

impl<'input> State<'input, '_> {
    fn need_to_open_valve(&self, valve: Valve<'_>) -> bool {
        valve != "AA" && !self.released_valves.contains(valve)
    }

    fn flow_rate_from_released_valves(&self) -> u64 {
        self.released_valves
            .iter()
            .map(|released_valve| self.nodes[released_valve].flow_rate)
            .sum()
    }

    fn open_valve(&mut self, valve: Valve<'input>) {
        assert!(!self.released_valves.contains(valve));
        self.released_valves.insert(valve);
    }

    fn advance_time_by_one_minute(&mut self) {
        assert!(self.time_left > 0);

        self.flow += self.flow_rate_from_released_valves();
        self.time_left -= 1;
    }

    fn all_valves_opened(&self) -> bool {
        self.nodes
            .iter()
            .filter(|&(valve, _)| valve != &"AA")
            .all(|(valve, node)| {
                assert!(node.flow_rate > 0);
                self.released_valves.contains(valve)
            })
    }

    fn out_of_time(&self) -> bool {
        self.time_left == 0
    }

    fn update_searcher(&mut self, searcher: Searcher<'input>) -> Searcher<'input> {
        match searcher {
            Searcher::AtValve(valve) => Searcher::AtValve(valve),
            Searcher::InTunnel {
                destination,
                time_until_arrival,
            } => {
                if time_until_arrival == 0 {
                    if self.need_to_open_valve(destination) {
                        self.open_valve(destination);
                    }
                    Searcher::AtValve(destination)
                } else {
                    Searcher::InTunnel {
                        destination,
                        time_until_arrival: time_until_arrival - 1,
                    }
                }
            }
        }
    }
}

fn search<'input, 'global>(mut state: State<'input, 'global>) -> State<'input, 'global> {
    if state.time_left == 0 {
        return state;
    }

    state.me = state.update_searcher(state.me);
    state.elephant = state.update_searcher(state.elephant);
    state.advance_time_by_one_minute();

    if state.all_valves_opened() {
        while !state.out_of_time() {
            state.advance_time_by_one_minute();
        }
        return state;
    }

    let nodes_left_to_visit = state
        .nodes
        .iter()
        .filter(|&(valve, node)| {
            let valve_already_released = state.released_valves.contains(valve);
            if valve_already_released {
                return false;
            }

            let being_visited_already =
                [&state.me, &state.elephant]
                    .iter()
                    .any(|searcher| match searcher {
                        Searcher::AtValve(position) => position == valve,
                        Searcher::InTunnel { destination, .. } => destination == valve,
                    });
            if being_visited_already {
                return false;
            }

            let can_be_reached_in_time =
                [&state.me, &state.elephant]
                    .iter()
                    .any(|searcher| match searcher {
                        Searcher::AtValve(position) => {
                            state.nodes[position].tunnels[valve] <= state.time_left
                        }
                        Searcher::InTunnel {
                            destination,
                            time_until_arrival,
                        } => {
                            state.nodes[destination].tunnels[valve] + time_until_arrival
                                <= state.time_left
                        }
                    });
            if !can_be_reached_in_time {
                return false;
            }

            node.flow_rate > 0
        })
        .collect::<Vec<_>>();

    if nodes_left_to_visit.is_empty() {
        return search(state);
    }

    match (state.me, state.elephant) {
        (Searcher::AtValve(me), Searcher::AtValve(elephant)) => {
            if nodes_left_to_visit.len() == 1 {
                let (&destination, _) = nodes_left_to_visit[0];

                search(State {
                    me: Searcher::InTunnel {
                        destination,
                        time_until_arrival: state.nodes[me].tunnels[destination],
                    },
                    elephant: Searcher::InTunnel {
                        destination,
                        time_until_arrival: state.nodes[me].tunnels[destination],
                    },
                    ..state.clone()
                })
            } else {
                nodes_left_to_visit
                    .into_iter()
                    .permutations(2)
                    .par_bridge()
                    .map(|permutation| {
                        let (my_destination, _) = permutation[0];
                        let (elephant_destination, _) = permutation[1];

                        search(State {
                            me: Searcher::InTunnel {
                                destination: my_destination,
                                time_until_arrival: state.nodes[me].tunnels[my_destination],
                            },
                            elephant: Searcher::InTunnel {
                                destination: elephant_destination,
                                time_until_arrival: state.nodes[elephant].tunnels
                                    [elephant_destination],
                            },
                            ..state.clone()
                        })
                    })
                    .max()
                    .unwrap()
            }
        }
        (Searcher::AtValve(me), elephant @ Searcher::InTunnel { .. }) => nodes_left_to_visit
            .into_iter()
            .par_bridge()
            .map(|(&valve, node)| {
                search(State {
                    me: Searcher::InTunnel {
                        destination: valve,
                        time_until_arrival: state.nodes[me].tunnels[valve],
                    },
                    elephant,
                    ..state.clone()
                })
            })
            .max()
            .unwrap(),
        (me @ Searcher::InTunnel { .. }, Searcher::AtValve(elephant)) => nodes_left_to_visit
            .into_iter()
            .par_bridge()
            .map(|(&valve, node)| {
                search(State {
                    me,
                    elephant: Searcher::InTunnel {
                        destination: valve,
                        time_until_arrival: state.nodes[elephant].tunnels[valve],
                    },
                    ..state.clone()
                })
            })
            .max()
            .unwrap(),
        (me @ Searcher::InTunnel { .. }, elephant @ Searcher::InTunnel { .. }) => search(state),
    }
}

fn simplify_graph<'input>(
    inputs: HashMap<Valve<'input>, Input<'input>>,
) -> HashMap<Valve<'input>, Node<'input>> {
    let mut nodes = HashMap::<Valve, Node>::new();
    for (current, _) in &inputs {
        let mut unvisited = inputs.keys().copied().collect::<HashSet<_>>();

        let mut distances = unvisited
            .iter()
            .copied()
            .map(|valve| (valve, None))
            .collect::<HashMap<Valve, Option<i32>>>();

        distances.insert(*current, Some(0));

        while !unvisited.is_empty() {
            let current = unvisited
                .iter()
                .copied()
                .min_by_key(|valve| distances[valve].unwrap_or(i32::MAX))
                .unwrap();

            unvisited.remove(&current);

            for neighbour in &inputs[current].tunnels {
                match (distances[current], distances[neighbour]) {
                    (Some(current_distance), Some(neighbour_distance)) => {
                        if current_distance + 1 < neighbour_distance {
                            distances.insert(*neighbour, Some(current_distance + 1));
                        }
                    }
                    (Some(current_distance), None) => {
                        distances.insert(*neighbour, Some(current_distance + 1));
                    }
                    _ => panic!("?"),
                }
            }
        }

        if current == &"AA" || inputs[current].flow_rate > 0 {
            nodes.insert(
                current,
                Node {
                    valve: current,
                    flow_rate: inputs[current].flow_rate,
                    tunnels: distances
                        .into_iter()
                        .filter_map(|(valve, cost)| {
                            if &valve == current {
                                return None;
                            }

                            let cost = cost.unwrap();
                            (valve == "AA" || cost != 0 && inputs[valve].flow_rate > 0)
                                .then_some((valve, cost))
                        })
                        .collect(),
                },
            );
        }
    }
    nodes
}

fn main() {
    let inputs = include_str!("input.txt")
        .lines()
        .map(parsing::parse_input)
        .map(|node| (node.valve, node))
        .collect::<HashMap<Valve, Input>>();

    let nodes = simplify_graph(inputs);

    let start = nodes
        .iter()
        .find_map(|(&valve, node)| (valve == "AA").then_some(node))
        .unwrap();

    let max_flow = search(State {
        me: Searcher::AtValve("AA"),
        elephant: Searcher::AtValve("AA"),
        time_left: 26,
        flow: start.flow_rate,
        released_valves: HashSet::default(),
        nodes: &nodes,
    })
    .flow;

    println!("Part 2: {max_flow}");

    // let max_flow = search_with_elephant(State {
    //     time_left: 26,
    //     flow: start.flow_rate,
    //     released_valves: HashSet::default(),
    //     all_nodes: &nodes,
    // })
    // .flow;
    //
    // println!("Part 2: {max_flow}");
}
