use std::{ops::Add, collections::{HashMap, BTreeSet, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::utils::get_input;


type Coordinate = (isize, isize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    Northeast,
    Northwest,
    South,
    Southwest,
    Southeast,
    East,
    West,
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => (self.0, self.1 - 1),
            Direction::South => (self.0, self.1 + 1),
            Direction::East =>  (self.0 + 1, self.1),
            Direction::West =>  (self.0 - 1, self.1),
            Direction::Northeast => (self.0 + 1, self.1 - 1),
            Direction::Northwest => (self.0 - 1, self.1 - 1),
            Direction::Southeast => (self.0 + 1, self.1 + 1),
            Direction::Southwest => (self.0 - 1, self.1 + 1),
        }
    }
}

fn square(elf: &Coordinate) -> [Coordinate; 8] {
    [
        *elf + Direction::North,
        *elf + Direction::Northeast,
        *elf + Direction::Northwest,
        *elf + Direction::South,
        *elf + Direction::Southeast,
        *elf + Direction::Southwest,
        *elf + Direction::East,
        *elf + Direction::West,
    ]
}

fn get_suggestion(elf: &Coordinate, elves: &BTreeSet<Coordinate>, idx: usize) -> Option<Coordinate> {
    // println!("Trying elf {:?}", elf);
    if !square(elf).iter().any(|elf| elves.contains(elf)) {
        //println!("Elf has no neighbors");
        return None;
    }
    let mut checks = [
        [*elf + Direction::North, *elf + Direction::Northeast, *elf + Direction::Northwest],
        [*elf + Direction::South, *elf + Direction::Southeast, *elf + Direction::Southwest],
        [*elf + Direction::West, *elf + Direction::Northwest, *elf + Direction::Southwest],
        [*elf + Direction::East, *elf + Direction::Southeast, *elf + Direction::Northeast],
    ];
    checks.rotate_left(idx);
    //println!("{:?}", elf);
    //println!("{:?}", checks);
    for check in checks {
        if check.into_iter().all(|c| !elves.contains(&c)) {
            //println!("PASSED: elf {:?} in direction {:?} {:?}", elf, dir, check[0]);
            return Some(check[0]);
        }
    }
    //println!("Elf has no viable options");
    None
}

// fn print_board(elves: &HashSet<Coordinate>) {
//     let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
//     let max_y = elves.iter().map(|&(_, y)| y).max().unwrap();
//     let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
//     let max_x = elves.iter().map(|&(x, _)| x).max().unwrap();
//     for y in (min_y - 1)..=(max_y + 1) {
//         for x in (min_x - 1)..=(max_x + 1) {
//             let tile = if elves.contains(&(x, y)) {
//                 '#'
//             } else {
//                 '.'
//             };
//             print!("{}", tile);
//         }
//         println!();
//     }
// }

fn score(elves: &BTreeSet<Coordinate>) -> usize {
    let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = elves.iter().map(|&(_, y)| y).max().unwrap();
    let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = elves.iter().map(|&(x, _)| x).max().unwrap();
    (max_y - min_y + 1) as usize * (max_x - min_x + 1) as usize - elves.len()
}

fn hash_elves(elves: &BTreeSet<Coordinate>) -> u64 {
    let mut hasher = DefaultHasher::new();
    elves.hash(&mut hasher);
    hasher.finish()
}

pub fn task1() {
    let elves: BTreeSet<Coordinate> = get_input(23)
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, tile)| if tile == '#' { Some((x as isize, y as isize))} else { None })
        })
        .collect();
    let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
    let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
    let mut elves: BTreeSet<Coordinate> = elves
        .into_iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect();
    // println!("{:?}", elves);
    // Map of destination coordinate to original elf. If we find a collision, both elves
    // are reset to their original locations
    let mut suggestions: HashMap<Coordinate, Coordinate> = HashMap::new();
    for round in 0..10 {
        for elf in &elves {
            // IF we have a suggestion, mark it; otherwise, just make it our
            // own and continue
            let Some(sugg) = get_suggestion(&elf, &elves, round % 4) else {
                if suggestions.insert(*elf, *elf).is_some() {
                    panic!();
                }
                continue;
            };
            // suggestion collision check - if collide, BOTH get reset!
            if let Some(&orig) = suggestions.get(&sugg) {
                // We have a collision!
                suggestions.remove(&sugg);
                suggestions.insert(orig, orig);
                suggestions.insert(*elf, *elf);
            } else {
                // No collision! Add our suggestion
                suggestions.insert(sugg, *elf);
            }
        }
        //TODO: Reset elves to suggestion results!
        elves = suggestions.into_keys().collect();
        suggestions = HashMap::new();
        // println!("After Round {}:", round + 1);
        // print_board(&elves);
    }
    println!("{}", score(&elves));
}

pub fn task2() {
    let elves: BTreeSet<Coordinate> = get_input(23)
    .lines()
    .enumerate()
    .flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, tile)| if tile == '#' { Some((x as isize, y as isize))} else { None })
    })
    .collect();
let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
let mut elves: BTreeSet<Coordinate> = elves
    .into_iter()
    .map(|(x, y)| (x - min_x, y - min_y))
    .collect();
// println!("{:?}", elves);
// Map of destination coordinate to original elf. If we find a collision, both elves
// are reset to their original locations
let mut suggestions: HashMap<Coordinate, Coordinate> = HashMap::new();
let mut hash = hash_elves(&elves);
for round in 0.. {
    for elf in &elves {
        // IF we have a suggestion, mark it; otherwise, just make it our
        // own and continue
        let Some(sugg) = get_suggestion(&elf, &elves, round % 4) else {
            if suggestions.insert(*elf, *elf).is_some() {
                panic!();
            }
            continue;
        };
        // suggestion collision check - if collide, BOTH get reset!
        if let Some(&orig) = suggestions.get(&sugg) {
            // We have a collision!
            suggestions.remove(&sugg);
            suggestions.insert(orig, orig);
            suggestions.insert(*elf, *elf);
        } else {
            // No collision! Add our suggestion
            suggestions.insert(sugg, *elf);
        }
    }
    //TODO: Reset elves to suggestion results!
    elves = suggestions.into_keys().collect();
    let new_hash = hash_elves(&elves);
    if hash == new_hash {
        println!("{}", round + 1);
        break;
    } else {
        hash = new_hash;
    }
    suggestions = HashMap::new();
    // println!("After Round {}:", round + 1);
    // print_board(&elves);
}
}