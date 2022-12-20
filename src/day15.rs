use std::collections::HashSet;

use regex::Regex;

use crate::utils::get_input;

fn distance(p1: &(isize, isize), p2: &(isize, isize)) -> isize {
    p1.0.abs_diff(p2.0) as isize + p1.1.abs_diff(p2.1) as isize
}

fn y_cover(y: isize, sensor: &(isize, isize), dist: &isize) -> Option<(isize, isize)> {
    let num_cols = dist - (sensor.1.abs_diff(y)) as isize;
    if num_cols <= 0 {
        None
    } else {
        // Note:
        Some((
            (sensor.0 - num_cols).max(0),
            (sensor.0 + num_cols + 1).min(4000000),
        ))
    }
}

pub fn task1() {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let sensors: Vec<((isize, isize), (isize, isize))> = get_input(15)
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|cap| {
            (
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            )
        })
        .collect();

    let mut pts = HashSet::new();
    let mut beacons = HashSet::new();
    for (sensor, beacon) in &sensors {
        beacons.insert(*beacon);
        // all points that are within the distance are fair game
        let d = distance(sensor, beacon);
        // naive way - create a square of this distance, then pare down to the ones in distance
        if !(sensor.1 - d..=sensor.1 + d).contains(&2000000) {
            continue;
        }
        for x in sensor.0 - d..=sensor.0 + d {
            for y in 2000000..=2000000 {
                if distance(sensor, &(x, y)) <= d && &(x, y) != beacon {
                    pts.insert((x, y));
                }
            }
        }
    }
    println!("{}", pts.iter().filter(|&&(_, y)| y == 2000000).count());
}

pub fn task2() {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let sensors: Vec<((isize, isize), isize)> = get_input(15)
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|cap| {
            let sensor = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            let beacon = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
            (sensor, distance(&sensor, &beacon))
        })
        .collect();
    // println!("   000000000011111111112");
    // println!("   012345678901234567890");
    for y in 0..=4000000 {
        let mut ranges: Vec<(isize, isize)> = sensors
            .iter()
            .flat_map(|(sensor, d)| y_cover(y, sensor, d))
            .collect();
        ranges.sort_unstable_by_key(|r| r.0);
        let mut x: isize = 0;
        //let mut s = vec!['.'; 21];

        for (start, end) in ranges {
            if x < start {
                println!(
                    "({}, {}) -> {}",
                    x,
                    y,
                    (x as usize) * 4000000 + (y as usize)
                );
                return;
            }
            // for c in x..start {
            //     s[c as usize] = '.';
            // }
            // for c in start..end {
            //     s[c as usize] = '#';
            // }
            x = x.max(end);
        }
        //println!("{:02} {}", y, s.iter().collect::<String>());
    }
}
