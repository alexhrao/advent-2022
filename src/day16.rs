use std::collections::HashMap;

use regex::Regex;

use crate::utils::get_input;

// NOTE: I did not come up with this myself; it was heavily influenced by
// https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day16/main.rs
// Hats of to them!

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

fn my_floyd_warshall(valves: &HashMap<String, Valve>) -> Vec<Vec<u128>> {
    // We're constructing a distance matrix
    // First, we need to get our row index
    let valve_idx: HashMap<&String, usize> = valves
        .values()
        .enumerate()
        .map(|(i, Valve { name, .. })| (name, i))
        .collect();
    // The distance matrix
    let mut dist_matrix = vec![vec![u128::MAX; valves.len()]; valves.len()];
    for (i, Valve { tunnels, .. }) in valves.values().enumerate() {
        for tunnel in tunnels {
            dist_matrix[i][valve_idx[tunnel]] = 1;
        }
    }
    // The diagonal should all be 0!
    for i in 0..valves.len() {
        dist_matrix[i][i] = 0;
    }
    // The fun part - O(V^3) according to Wikipedia
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                let (d, o) = dist_matrix[i][k].overflowing_add(dist_matrix[k][j]);
                // If we haven't overflowed (IDK how that would be possible without a negative cycle!)
                if !o && dist_matrix[i][j] > d {
                    dist_matrix[i][j] = d;
                }
            }
        }
    }
    dist_matrix
}

fn floyd_warshall(rows: &[(&str, u8, Vec<&str>)]) -> Vec<Vec<u8>> {
    let valve_name_to_idx: HashMap<&str, usize> = rows
        .iter()
        .enumerate()
        .map(|(i, &(name, _, _))| (name, i))
        .collect();

    let mut dist = vec![vec![u8::MAX; rows.len()]; rows.len()];
    for (i, (_, _, tunnels)) in rows.iter().enumerate() {
        for tunnel in tunnels {
            let j = valve_name_to_idx[tunnel];
            dist[i][j] = 1;
        }
    }
    (0..dist.len()).for_each(|i| {
        dist[i][i] = 0;
    });
    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
                if !overflow && dist[i][j] > result {
                    dist[i][j] = result;
                }
            }
        }
    }
    dist
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
}

pub fn task2() {}
