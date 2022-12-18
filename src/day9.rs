use std::collections::HashSet;

use regex::Regex;

use crate::utils::get_input;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        }
    }
}

impl std::ops::AddAssign<Direction> for (i128, i128) {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
            Direction::None => (),
        };
    }
}

fn are_touching(head: &(i128, i128), tail: &(i128, i128)) -> bool {
    (head.0 - tail.0).abs() < 2 && (head.1 - tail.1).abs() < 2
}

pub fn task1() {
    let mut visited = HashSet::new();

    let mut head = (0, 0);
    let mut tail = head;
    visited.insert(tail);

    let step_re = Regex::new(r"^([UDLR]) (\d+)$").unwrap();

    for step in get_input(9).lines() {
        let caps = step_re.captures(step).unwrap();
        let num_steps: u128 = caps[2].parse().unwrap();
        let dir = caps[1].into();

        for _ in 0..num_steps {
            head += dir;
            if !are_touching(&head, &tail) {
                // Figure out the direction
                let xx = head.0 - tail.0;
                let yy = head.1 - tail.1;
                tail += if xx.is_negative() {
                    Direction::Left
                } else if xx.is_positive() {
                    Direction::Right
                } else {
                    Direction::None
                };
                tail += if yy.is_negative() {
                    Direction::Down
                } else if yy.is_positive() {
                    Direction::Up
                } else {
                    Direction::None
                };
            }

            visited.insert(tail);
        }
    }
    println!("{}", visited.len());
}

pub fn task2() {
    let mut visited = HashSet::new();

    let mut rope = [(0, 0); 10];
    visited.insert(rope[9]);

    let step_re = Regex::new(r"^([UDLR]) (\d+)$").unwrap();

    for step in get_input(9).lines() {
        let caps = step_re.captures(step).unwrap();
        let num_steps: u128 = caps[2].parse().unwrap();
        let dir = caps[1].into();

        for _ in 0..num_steps {
            rope[0] += dir;
            for r in 1..10 {
                if !are_touching(&rope[r-1], &rope[r]) {
                    // Figure out the direction
                    let xx = rope[r-1].0 - rope[r].0;
                    let yy = rope[r-1].1 - rope[r].1;
                    rope[r] += if xx.is_negative() {
                        Direction::Left
                    } else if xx.is_positive() {
                        Direction::Right
                    } else {
                        Direction::None
                    };
                    rope[r] += if yy.is_negative() {
                        Direction::Down
                    } else if yy.is_positive() {
                        Direction::Up
                    } else {
                        Direction::None
                    };
                }
            }
            visited.insert(rope[9]);
        }
    }
    println!("{}", visited.len());
}
