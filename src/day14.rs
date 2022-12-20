use std::collections::HashSet;

use regex::Regex;

use crate::utils::get_input;

pub fn task1() {
    let pt_re = Regex::new(r"(\d+),(\d+)").unwrap();
    let segments: Vec<Vec<(usize, usize)>> = get_input(14)
        .lines()
        .map(|l| {
            pt_re
                .captures_iter(l)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .collect()
        })
        .collect();

    let mut points = HashSet::new();
    for seg in segments {
        for (&(x1, y1), &(x2, y2)) in seg[..seg.len() - 1].iter().zip(seg[1..].iter()) {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    points.insert((x1, y));
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    points.insert((x, y1));
                }
            }
        }
    }
    let mut sand_count = 0;
    let bottom_level = points.iter().map(|&(_, y)| y).max().unwrap();
    // Start the simulation
    loop {
        // drop a sand grain
        sand_count += 1;
        let (mut x, mut y) = (500, 0);
        // See where it falls!
        if loop {
            // look at down, then down-left, then down-right; otherwise, this is as far as we go!
            if !points.contains(&(x, y + 1)) {
                y += 1;
            } else if !points.contains(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
            } else if !points.contains(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
            } else {
                // We've stopped!
                points.insert((x, y));
                break false;
            }

            if y > bottom_level {
                break true;
            }
        } {
            sand_count -= 1;
            break;
        }
    }
    println!("{}", sand_count);
}
pub fn task2() {
    let pt_re = Regex::new(r"(\d+),(\d+)").unwrap();
    let segments: Vec<Vec<(usize, usize)>> = get_input(14)
        .lines()
        .map(|l| {
            pt_re
                .captures_iter(l)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .collect()
        })
        .collect();

    let mut points = HashSet::new();
    for seg in segments {
        for (&(x1, y1), &(x2, y2)) in seg[..seg.len() - 1].iter().zip(seg[1..].iter()) {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    points.insert((x1, y));
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    points.insert((x, y1));
                }
            }
        }
    }
    let mut sand_count = 0;
    let bottom_level = points.iter().map(|&(_, y)| y).max().unwrap() + 2;
    // Start the simulation
    loop {
        // drop a sand grain
        sand_count += 1;
        let (mut x, mut y) = (500, 0);
        // See where it falls!
        if loop {
            // look at down, then down-left, then down-right; otherwise, this is as far as we go!
            if (y + 1) == bottom_level {
                points.insert((x, y));
                break false;
            } else if !points.contains(&(x, y + 1)) {
                y += 1;
            } else if !points.contains(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
            } else if !points.contains(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
            } else {
                // We've stopped!
                points.insert((x, y));
                break y == 0 && x == 500;
            }
        } {
            break;
        }
    }
    println!("{}", sand_count);
}
