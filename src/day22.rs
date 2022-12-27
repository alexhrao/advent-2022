use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::{Add, AddAssign},
};

use regex::Regex;

use crate::utils::get_input;

type RC = (usize, usize);
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Open,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    CW,
    CCW,
    None,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Direction::CW,
            'L' => Direction::CCW,
            'N' => Direction::None,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    Right,
    Down,
    Left,
    Up,
}

impl From<isize> for Orientation {
    fn from(value: isize) -> Self {
        match value.rem_euclid(4) {
            0 => Orientation::Right,
            1 => Orientation::Down,
            2 => Orientation::Left,
            3 => Orientation::Up,
            _ => unreachable!(),
        }
    }
}

impl From<Orientation> for isize {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
            Orientation::Up => 3,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Orientation::Right => '>',
            Orientation::Down => 'v',
            Orientation::Left => '<',
            Orientation::Up => '^',
        })
    }
}

impl Add<Direction> for Orientation {
    type Output = Orientation;
    fn add(self, rhs: Direction) -> Self::Output {
        let a: isize = self.into();
        match rhs {
            Direction::CW => (a + 1).into(),
            Direction::CCW => (a - 1).into(),
            Direction::None => a.into(),
        }
    }
}

impl AddAssign<Direction> for Orientation {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Add<Orientation> for RC {
    type Output = RC;
    fn add(self, rhs: Orientation) -> Self::Output {
        match rhs {
            Orientation::Right => (self.0, self.1 + 1),
            Orientation::Down => (self.0 + 1, self.1),
            Orientation::Left => (self.0, self.1 - 1),
            Orientation::Up => (self.0 - 1, self.1),
        }
    }
}

type PartyLocation = (RC, Orientation);

fn wrap(party: &PartyLocation, tiles: &HashMap<RC, Tile>) -> RC {
    *match party.1 {
        Orientation::Down => tiles
            .keys()
            .filter(|&&x| x.1 == party.0 .1)
            .min_by_key(|&&(r, _)| r)
            .unwrap(), // the min row that has our column
        Orientation::Right => tiles
            .keys()
            .filter(|&&x| x.0 == party.0 .0)
            .min_by_key(|&&(_, c)| c)
            .unwrap(), // the min column that has our row
        Orientation::Up => tiles
            .keys()
            .filter(|&&x| x.1 == party.0 .1)
            .max_by_key(|&&(r, _)| r)
            .unwrap(), // the max row that has our column
        Orientation::Left => tiles
            .keys()
            .filter(|&&x| x.0 == party.0 .0)
            .max_by_key(|&&(_, c)| c)
            .unwrap(), // the max column that has our row
    }
}

fn next_step(party: &PartyLocation, tiles: &HashMap<RC, Tile>) -> RC {
    let mut test = party.0 + party.1;
    // Note: Rows start AT 1! This is so we can safely subtract. 0 is always considered a wall
    if !tiles.contains_key(&test) {
        // try to wrap around. To do that, find the "highest" value in the opposite orientation:
        test = wrap(party, tiles);
    }

    // now we have a test value that we know EXISTS on the board, although
    // it could totally be a wall. If it's OPEN, move; otherwise, just return
    // our starting location
    if let Some(Tile::Open) = tiles.get(&test) {
        test
    } else {
        party.0
    }
}

fn abs_to_off(abs: RC, side_len: usize) -> RC {
    let face = get_face(&abs, side_len);
    let (r, c) = abs;
    match face {
        1 => (r, c - side_len),
        2 => (r, c - 2 * side_len),
        3 => (r - side_len, c - side_len),
        4 => (r - 2 * side_len, c),
        5 => (r - 2 * side_len, c - side_len),
        6 => (r - 3 * side_len, c),
        _ => unreachable!(),
    }
}

fn off_to_abs(off: RC, face: u8, side_len: usize) -> RC {
    let (r, c) = off;
    match face {
        1 => (r, c + side_len),
        2 => (r, c + 2 * side_len),
        3 => (r + side_len, c + side_len),
        4 => (r + 2 * side_len, c),
        5 => (r + 2 * side_len, c + side_len),
        6 => (r + 3 * side_len, c),
        _ => unreachable!(),
    }
}

fn wrap_cube(party: &PartyLocation, side_len: usize) -> PartyLocation {
    // get the current face
    let curr_face = get_face(&party.0, side_len);
    let next = next_face(curr_face, party.1);
    let (off_r, off_c) = abs_to_off(party.0, side_len);
    if next.1 == party.1 {
        // same direction - same offset in one axis; opposite in the other
        let off = match next.1 {
            Orientation::Right => (off_r, 1),       // column reset to 1
            Orientation::Down => (1, off_c),        // row reset to 1
            Orientation::Left => (off_r, side_len), // col reset to max
            Orientation::Up => (side_len, off_c),   // row reset to max
        };
        return (off_to_abs(off, next.0, side_len), next.1);
    } else {
        let off = match (curr_face, next.0) {
            (1, 2)
            | (1, 3)
            | (3, 5)
            | (4, 5)
            | (4, 6)
            | (2, 1)
            | (3, 1)
            | (5, 3)
            | (5, 4)
            | (6, 4) => unreachable!(),
            (1, 4) => (side_len - off_r + 1, 1),
            (4, 1) => (side_len - off_r + 1, 1),
            (1, 6) => (off_c, 1),
            (6, 1) => (1, off_r),
            (2, 3) => (off_c, side_len),
            (3, 2) => (side_len, off_r),
            (2, 5) => (side_len - off_r + 1, side_len),
            (5, 2) => (side_len - off_r + 1, side_len),
            (3, 4) => (1, off_r),
            (4, 3) => (off_c, 1),
            (5, 6) => (off_c, side_len),
            (6, 5) => (side_len, off_r),
            _ => unreachable!(),
        };

        return (off_to_abs(off, next.0, side_len), next.1);
    }
}

fn next_cube_step(
    party: &PartyLocation,
    tiles: &HashMap<RC, Tile>,
    side_len: usize,
) -> PartyLocation {
    let party = *party;
    let mut test = (party.0 + party.1, party.1);
    // Note: Rows start AT 1! This is so we can safely subtract. 0 is always considered a wall
    if !tiles.contains_key(&test.0) {
        // try to wrap around. To do that, find the "highest" value in the opposite orientation:
        test = wrap_cube(&party, side_len);
    }

    // now we have a test value that we know EXISTS on the board, although
    // it could totally be a wall. If it's OPEN, move; otherwise, just return
    // our starting location
    if let Some(Tile::Open) = tiles.get(&test.0) {
        test
    } else {
        party
    }
}

fn next_face(face: u8, dir: Orientation) -> (u8, Orientation) {
    let scale = if face > 3 { 3 } else { 0 };
    let face = face - scale;
    use Orientation::*;
    match (face, dir) {
        (1, Right) => (2 + scale, Right),
        (1, Down) => (3 + scale, Down),
        (1, Left) => (((4 + scale - 1) % 6) + 1, Right),
        (1, Up) => (((6 + scale - 1) % 6) + 1, Right),

        (2, Right) => (((5 + scale - 1) % 6) + 1, Left),
        (2, Down) => (3 + scale, Left),
        (2, Left) => (((7 + scale - 1) % 6) + 1, Left),
        (2, Up) => (((6 + scale - 1) % 6) + 1, Up),

        (3, Right) => (((8 + scale - 1) % 6) + 1, Up),
        (3, Down) => (((5 + scale - 1) % 6) + 1, Down),
        (3, Left) => (((4 + scale - 1) % 6) + 1, Down),
        (3, Up) => (((7 + scale - 1) % 6) + 1, Up),
        _ => unreachable!(),
    }
}

fn get_face(rc: &RC, side_len: usize) -> u8 {
    let &(r, c) = rc;
    let rc = ((r - 1) / side_len, (c - 1) / side_len);
    match rc {
        (0, 1) => 1,
        (0, 2) => 2,
        (1, 1) => 3,
        (2, 0) => 4,
        (2, 1) => 5,
        (3, 0) => 6,
        _ => unreachable!(),
    }
}

fn draw_map(tiles: &HashMap<RC, Tile>, locs: &HashMap<RC, Orientation>, last: &RC) {
    let max_r = tiles.keys().map(|&(r, _)| r).max().unwrap();
    let max_c = tiles.keys().map(|&(_, c)| c).max().unwrap();
    for r in 1..=max_r {
        for c in 1..=max_c {
            if !tiles.contains_key(&(r, c)) {
                print!(" ");
            } else if let Tile::Wall = tiles[&(r, c)] {
                print!("#");
            } else if &(r, c) == last {
                print!("*");
            } else if let Some(dir) = locs.get(&(r, c)) {
                print!("{}", *dir);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn task1() {
    let tiles: HashMap<RC, Tile> = get_input(22)
        .lines()
        .take_while(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars().enumerate().filter_map(move |(c, t)| {
                if t == ' ' {
                    None
                } else {
                    Some(((r + 1, c + 1), t.into()))
                }
            })
        })
        .collect();
    let re = Regex::new(r"(\d+)([RL]?)").unwrap();
    let instructions: Vec<(u128, Direction)> = re
        .captures_iter(get_input(22).lines().last().unwrap())
        .map(|cap| {
            (
                cap[1].parse().unwrap(),
                cap[2].chars().nth(0).unwrap_or('N').into(),
            )
        })
        .collect();
    let min_r = tiles
        .iter()
        .filter(|&(_, &t)| t == Tile::Open)
        .min_by_key(|&(&(r, _), _)| r)
        .unwrap()
        .0
         .0;

    let mut curr_location: PartyLocation = (
        *tiles
            .iter()
            .filter(|&(&(r, _), &t)| t == Tile::Open && r == min_r)
            .min_by_key(|&(&(_, c), _)| c)
            .unwrap()
            .0,
        Orientation::Right,
    );
    let mut locs: HashMap<RC, Orientation> = HashMap::new();
    let mut last_loc = (0, 0);
    for (steps, dir) in instructions {
        for _ in 0..steps {
            locs.insert(curr_location.0, curr_location.1);
            last_loc = curr_location.0;
            curr_location.0 = next_step(&curr_location, &tiles);
        }
        curr_location.1 += dir;
    }
    if false {
        draw_map(&tiles, &locs, &last_loc);
    }
    println!(
        "{}",
        curr_location.0 .0 * 1000 + 4 * curr_location.0 .1 + curr_location.1 as usize
    );
}

pub fn task2() {
    let tiles: HashMap<RC, Tile> = get_input(22)
        .lines()
        .take_while(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars().enumerate().filter_map(move |(c, t)| {
                if t == ' ' {
                    None
                } else {
                    Some(((r + 1, c + 1), t.into()))
                }
            })
        })
        .collect();
    let re = Regex::new(r"(\d+)([RL]?)").unwrap();
    let instructions: Vec<(u128, Direction)> = re
        .captures_iter(get_input(22).lines().last().unwrap())
        .map(|cap| {
            (
                cap[1].parse().unwrap(),
                cap[2].chars().nth(0).unwrap_or('N').into(),
            )
        })
        .collect();
    let side_len = tiles
        .keys()
        .map(|&(r, _)| r)
        .max()
        .unwrap()
        .max(tiles.keys().map(|&(_, c)| c).max().unwrap())
        / 4;
    let mut curr_location: PartyLocation = ((1, side_len + 1), Orientation::Right);
    let mut locs: HashMap<RC, Orientation> = HashMap::new();
    let mut last_loc: RC = (0, 0);
    for (steps, dir) in instructions {
        for _ in 0..steps {
            locs.insert(curr_location.0, curr_location.1);
            last_loc = curr_location.0;
            curr_location = next_cube_step(&curr_location, &tiles, side_len);
        }
        curr_location.1 += dir;
    }
    if false {
        draw_map(&tiles, &locs, &last_loc);
    }
    println!(
        "{}",
        curr_location.0 .0 * 1000 + 4 * curr_location.0 .1 + curr_location.1 as usize
    );
}

#[cfg(test)]
mod test {
    use crate::day22::*;
    #[test]
    fn abs_to_off_works() {
        assert_eq!((1, 1), abs_to_off((1, 51), 50));
        assert_eq!((1, 1), abs_to_off((1, 101), 50));
        assert_eq!((1, 1), abs_to_off((51, 51), 50));
        assert_eq!((1, 1), abs_to_off((101, 1), 50));
        assert_eq!((1, 1), abs_to_off((101, 51), 50));
        assert_eq!((1, 1), abs_to_off((151, 1), 50));
    }

    #[test]
    fn off_to_abs_works() {
        assert_eq!((1, 51), off_to_abs((1, 1), 1, 50));
        assert_eq!((1, 101), off_to_abs((1, 1), 2, 50));
        assert_eq!((51, 51), off_to_abs((1, 1), 3, 50));
        assert_eq!((101, 1), off_to_abs((1, 1), 4, 50));
        assert_eq!((101, 51), off_to_abs((1, 1), 5, 50));
        assert_eq!((151, 1), off_to_abs((1, 1), 6, 50));
    }
}
