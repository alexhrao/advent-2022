use std::{collections::HashSet, fmt::Display, ops::Add};

use crate::utils::get_input;

use pathfinding::prelude::bfs;

type Coordinate = (usize, usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '^' => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Blizzard {
    pub location: Coordinate,
    pub direction: Direction,
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Right => (self.0 + 1, self.1),
            Direction::Left => (self.0 - 1, self.1),
            Direction::Up => (self.0, self.1 - 1),
            Direction::Down => (self.0, self.1 + 1),
        }
    }
}

impl Blizzard {
    pub fn step(&mut self, dimensions: &(usize, usize)) {
        let test = self.location + self.direction;
        if test.0 == 0
            || test.0 == (dimensions.0 - 1)
            || test.1 == 0
            || test.1 == (dimensions.1 - 1)
        {
            self.location = match self.direction {
                Direction::Down => (self.location.0, 1),
                Direction::Right => (1, self.location.1),
                Direction::Left => (dimensions.0 - 2, self.location.1),
                Direction::Up => (self.location.0, dimensions.1 - 2),
            }
        } else {
            self.location = test;
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Direction::Right => ">",
                Direction::Left => "<",
                Direction::Up => "^",
                Direction::Down => "v",
            }
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct BoardState {
    pub minute: usize,
    pub party: Coordinate,
    pub blizzards: Vec<Blizzard>,
}

// fn print_board(state: &BoardState, dims: &(usize, usize), start: &Coordinate, end: &Coordinate) {
//     println!("-------");
//     println!("{:?}", state.party);
//     for y in 0..dims.1 {
//         print!("#");
//         for x in 1..(dims.0 - 1) {
//             if state.party == (x, y) {
//                 print!("E");
//                 continue;
//             } else if &(x, y) == start || &(x, y) == end {
//                 print!(".");
//                 continue;
//             } else if y == 0 || y == dims.1 - 1 {
//                 print!("#");
//                 continue;
//             }
//             let ct = state.blizzards.iter().filter(|b| b.location == (x, y)).count();
//             if ct == 0 {
//                 print!(".");
//             } else if ct == 1 {
//                 print!("{}", state.blizzards.iter().find(|b| b.location == (x, y)).unwrap().direction);
//             } else {
//                 print!("{}", ct);
//             }
//         }
//         println!("#");
//     }

//     println!("-------");
// }

pub fn task1() {
    let map = get_input(24);
    let dims = (map.lines().next().unwrap().len(), map.lines().count());
    let blizzards: Vec<_> = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let res: Result<Direction, ()> = c.try_into();
                if let Ok(direction) = res {
                    Some(Blizzard {
                        direction,
                        location: (x, y),
                    })
                } else {
                    None
                }
            })
        })
        .collect();
    let start = (
        map.lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap(),
        0usize,
    );
    let dest = (
        map.lines()
            .last()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap(),
        dims.1 - 1,
    );
    let seed = BoardState {
        minute: 0,
        party: start,
        blizzards,
    };

    let a = bfs(
        &seed,
        |board| {
            let mut starter = board.clone();
            starter.minute += 1;
            for blizzard in &mut starter.blizzards {
                blizzard.step(&dims);
            }
            let blizzards: HashSet<_> = starter.blizzards.iter().map(|b| b.location).collect();
            let party = board.party;
            // Could go up, left, right, down, or just stay in place
            let mut out = vec![];
            {
                let test = party + Direction::Left;
                if test.0 != 0 && (test.1 != 0 || test == start) && !blizzards.contains(&test) {
                    out.push(test);
                }
            }
            {
                let test = party + Direction::Right;
                if test.0 != (dims.0 - 1)
                    && (test.1 != 0 || test == start)
                    && !blizzards.contains(&test)
                {
                    out.push(test);
                }
            }
            if party.1 != 0 {
                let test = party + Direction::Up;
                if (test.1 != 0 || test == start) && !blizzards.contains(&test) {
                    out.push(test);
                }
            }
            {
                let test = party + Direction::Down;
                if (test.1 != dims.1 || test == dest) && !blizzards.contains(&test) {
                    out.push(test);
                }
            }
            if !blizzards.contains(&party) {
                out.push(party);
            }
            out.into_iter().map(move |party| BoardState {
                minute: starter.minute,
                party,
                blizzards: starter.blizzards.clone(),
            })
        },
        |x| x.party == dest,
    );
    let a = a.unwrap();
    let len = a.len();
    // 186 is too LOW
    println!("{}", len);
}

pub fn task2() {
    todo!();
}
