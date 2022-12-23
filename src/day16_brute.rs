use std::collections::{HashMap, HashSet};

use regex::Regex;
#[derive(Clone, Copy, Debug)]
enum Action<'a> {
    Traverse(&'a Valve),
    Open(&'a Valve),
}

use crate::utils::get_input;
#[derive(Debug, PartialEq, Eq)]
struct Valve {
    pub name: String,
    pub rate: u128,
    pub tunnels: Vec<String>,
}

impl Valve {
    pub fn new(name: String, rate: u128, tunnels: Vec<String>) -> Valve {
        Valve {
            name,
            rate,
            tunnels,
        }
    }
}

#[derive(Debug)]
struct History<'a> {
    pub actions: Vec<Action<'a>>,
    pub opened: HashSet<&'a String>,
}

impl<'a> History<'a> {
    pub fn get_flow(&self, valves: &HashMap<String, Valve>) -> u128 {
        let mut flow = 0;
        let mut opened: HashSet<&String> = HashSet::new();
        for action in &self.actions {
            // calculate the flow
            flow += opened.iter().map(|&vn| valves[vn].rate).sum::<u128>();
            if let Action::Open(v) = action {
                opened.insert(&v.name);
            }
        }

        flow
    }
}

fn explore<'a>(
    valve: &'a Valve,
    opened: &HashSet<&String>,
    valves: &'a HashMap<String, Valve>,
) -> Vec<Action<'a>> {
    let mut actions = Vec::with_capacity(valve.tunnels.len() + 1);
    if !opened.contains(&valve.name) {
        actions.push(Action::Open(valve));
    }
    actions.extend(valve.tunnels.iter().map(|t| Action::Traverse(&valves[t])));

    actions
    // I can either go down
}

pub fn task1() {
    let valve_re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.+)",
    )
    .unwrap();
    let data: Vec<_> = get_input(16)
        .lines()
        .flat_map(|l| valve_re.captures(l))
        .map(|c| (c[1].to_string(), c[2].to_string(), c[3].to_string()))
        .map(|(valve, rate, tunnels)| {
            (
                valve,
                rate.parse::<u128>().unwrap(),
                tunnels
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect();
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for (name, rate, tunnels) in data {
        valves.insert(name.clone(), Valve::new(name, rate, tunnels));
    }

    let valves = valves;

    let opened: HashSet<&String> = HashSet::new();

    let mut timelines: Vec<History> = explore(&valves["AA"], &opened, &valves)
        .into_iter()
        .map(|a| {
            let mut hs = HashSet::new();
            if let Action::Open(v) = a {
                hs.insert(&v.name);
            }
            History {
                actions: vec![a],
                opened: hs,
            }
        })
        .collect();
    for _ in 1..30 {
        // go through the past history. For each one, create the next chapter
        let mut next = vec![];
        for past in timelines.drain(..) {
            let valve = *match past.actions.last().unwrap() {
                Action::Open(v) | Action::Traverse(v) => v,
            };
            past.actions.last().unwrap();
            next.extend(explore(valve, &past.opened, &valves).into_iter().map(|a| {
                let mut hs = past.opened.clone();
                if let Action::Open(v) = a {
                    hs.insert(&v.name);
                }
                let mut actions: Vec<Action> = past.actions.iter().copied().collect();
                actions.push(a);
                History {
                    actions,
                    opened: hs,
                }
            }));
        }
        timelines = next;
    }
    let chosen = timelines
        .iter()
        .max_by_key(|h| h.get_flow(&valves))
        .unwrap();
    println!("{}", chosen.get_flow(&valves));
}

pub fn task2() {}
