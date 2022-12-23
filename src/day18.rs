use std::collections::{HashSet, BTreeSet};

use regex::Regex;

use crate::utils::get_input;
type Point = (u128, u128, u128);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Side {
    XY(Point),
    XZ(Point),
    YZ(Point),
}

pub fn task1() {
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let sides: Vec<Side> = get_input(18)
        .lines()
        .flat_map(|l| {
            let caps = re.captures(l).unwrap();
            let cube: Point = (caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap());
            [
                Side::XY(cube),
                Side::XZ(cube),
                Side::YZ(cube),
                Side::XY((cube.0, cube.1, cube.2 + 1)),
                Side::XZ((cube.0, cube.1 + 1, cube.2)),
                Side::YZ((cube.0 + 1, cube.1, cube.2)),
            ]
        })
        .collect();
    let mut seen = HashSet::new();
    for side in sides {
        if seen.contains(&side) {
            seen.remove(&side);
        } else {
            seen.insert(side);
        }
    }
    println!("{}", seen.len())
}

pub fn task2() {
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let squares: BTreeSet<BTreeSet<Side>> = get_input(18)
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let cube: Point = (caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap());
            [
                Side::XY(cube),
                Side::XZ(cube),
                Side::YZ(cube),
                Side::XY((cube.0, cube.1, cube.2 + 1)),
                Side::XZ((cube.0, cube.1 + 1, cube.2)),
                Side::YZ((cube.0 + 1, cube.1, cube.2)),
            ].into_iter().collect::<BTreeSet<Side>>()
        })
        .collect();
    let sides: Vec<Side> = squares.iter()
        .flat_map(|s| s.clone())
        .collect();

    let _points: Vec<Point> = get_input(18)
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap())
        }).collect();
    
    let mut seen = BTreeSet::new();

    for side in sides {
        if seen.contains(&side) {
            seen.remove(&side);
        } else {
            seen.insert(side);
        }
    }
    let mut to_remove: BTreeSet<Side> = BTreeSet::new();
    println!("Seen: {}", seen.len());
    println!("To Remove: {}", to_remove.len());
    for cube in &seen {
        let pt = *match cube {
            Side::XY(pt)|Side::XZ(pt)|Side::YZ(pt) => pt,
        };
        // for each side, see if we can reconstruct the square; if:
        // a) All the sides exist
        // b) It's not an actual square LMAO
        let sides = [
            Side::XY(pt),
            Side::XZ(pt),
            Side::YZ(pt),
            Side::XY((pt.0, pt.1, pt.2 + 1)),
            Side::XZ((pt.0, pt.1 + 1, pt.2)),
            Side::YZ((pt.0 + 1, pt.1, pt.2)),
        ];
        if sides.iter().all(|s| seen.contains(s)) {
            let cube: BTreeSet<Side> = sides.into_iter().collect();
            if !squares.contains(&cube) {
                to_remove.extend(sides.iter());
            }
        }
    }

    // 1824 is TOO LOW
    // 3272 is TOO HIGH
    // 3100 is TOO HIGH
    // LOL it was 2000...
    println!("To Remove: {}", to_remove.len());
    println!("Sides: {}", seen.len() - to_remove.len());
}
