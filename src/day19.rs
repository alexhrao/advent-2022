use std::fmt::Display;

use regex::Regex;

use crate::utils::get_input;
#[derive(Debug)]
struct Blueprint {
    pub id: usize,
    pub ore: u128,
    pub clay: u128,
    pub obsidian: (u128, u128),
    pub geode: (u128, u128),
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Blueprint { id, ore, clay, obsidian, geode } = self;
        f.write_fmt(format_args!("Blueprint {id}, {ore}, {clay}, {obsidian:?}, {geode:?}"))
    }
}

pub fn task1() {
    let blueprint_re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let prints: Vec<Blueprint> = get_input(19)
        .lines()
        .map(|l| {
            if let Some(caps) = blueprint_re.captures(l) {
                let id = caps[1].parse().unwrap();
                let ore = caps[2].parse().unwrap();
                let clay = caps[3].parse().unwrap();
                let obs_ore = caps[4].parse().unwrap();
                let obs_clay = caps[5].parse().unwrap();
                let geo_ore = caps[6].parse().unwrap();
                let geo_obs = caps[7].parse().unwrap();

                Blueprint {
                    id,
                    ore,
                    clay,
                    obsidian: (obs_ore, obs_clay),
                    geode: (geo_ore, geo_obs),
                }
            } else {
                panic!();
            }
        })
        .collect();

    println!("{:?}", prints);
}
