use std::{
    collections::{HashMap, HashSet},
    hint::unreachable_unchecked,
    ops::{Add, AddAssign},
};

use crate::utils::get_input;
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            c => {
                println!("{}", c);
                unreachable!()
            }
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
            Direction::Down => (self.0, self.1 - 1),
        }
    }
}

impl AddAssign<Direction> for (usize, usize) {
    fn add_assign(&mut self, rhs: Direction) {
        self.0 = match rhs {
            Direction::Left => self.0 - 1,
            Direction::Right => self.0 + 1,
            Direction::Down => self.0,
        };
        self.1 = match rhs {
            Direction::Left => self.1,
            Direction::Right => self.1,
            Direction::Down => self.1 - 1,
        };
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Shape {
    Dash,
    Plus,
    RightAngle,
    Pipe,
    Square,
}

impl Shape {
    pub fn nth(n: usize) -> Shape {
        match n % 5 {
            0 => Self::Dash,
            1 => Self::Plus,
            2 => Self::RightAngle,
            3 => Self::Pipe,
            4 => Self::Square,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn vertices(&self, bottom_left: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *bottom_left;
        match self {
            Shape::Dash => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Shape::Plus => vec![
                (x + 1, y + 2),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y),
            ],
            Shape::RightAngle => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Shape::Pipe => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Shape::Square => vec![(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)],
        }
    }

    pub fn top(&self, bottom: usize) -> usize {
        match self {
            Shape::Dash => bottom,
            Shape::Plus => bottom + 2,
            Shape::RightAngle => bottom + 2,
            Shape::Pipe => bottom + 3,
            Shape::Square => bottom + 1,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Piece {
    pub shape: Shape,
    pub bottom_left: (usize, usize),
}

impl Piece {
    pub fn would_intersect(&self, others: &[Piece], dir: &Direction) -> bool {
        // Check the walls & floor - no coord > 8, no coord < 0
        self.shape
            .vertices(&(self.bottom_left + *dir))
            .into_iter()
            .any(|(x, y)| x == 0 || x == 8 || y == 0)
            || others
                .iter()
                .filter(|p| p.shape.top(p.bottom_left.1) >= (self.bottom_left.1 - 1))
                .any(|p| self.would_intersect_with(p, dir))
    }
    fn would_intersect_with(&self, other: &Piece, dir: &Direction) -> bool {
        let mine = self.shape.vertices(&(self.bottom_left + *dir));
        let theirs = other.shape.vertices(&other.bottom_left);
        for (mx, my) in &mine {
            for (tx, ty) in &theirs {
                if tx == mx && ty == my {
                    return true;
                }
            }
        }
        false
    }
}

fn print_cave(piece: &Piece, placed: &[Piece]) {
    let y = placed.iter().map(|p| p.shape.top(p.bottom_left.1)).max();
    let y = if y.is_some() { y.unwrap() } else { 0 };
    let y = piece.shape.top(piece.bottom_left.1).max(y);
    let verts: HashSet<_> = placed
        .iter()
        .flat_map(|p| p.shape.vertices(&p.bottom_left))
        .collect();
    let my_verts = piece.shape.vertices(&piece.bottom_left);
    let my_verts: HashSet<_> = my_verts.iter().collect();
    for yy in (1..=y).rev() {
        print!("|");
        for xx in 1..=7 {
            if verts.contains(&(xx, yy)) {
                print!("#");
            } else if my_verts.contains(&(xx, yy)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("---------");
}

fn get_topology(placed: &[Piece]) -> [usize; 7] {
    let mut seen = [false; 7];
    let mut out = [0; 7];
    // Start by getting the tip top y
    let top_y = placed
        .iter()
        .map(|p| p.shape.top(p.bottom_left.1))
        .max()
        .unwrap_or(0);
    // get all the vertices for all shapes (Is there a way to optimize this?)
    let vertices: HashSet<_> = placed
        .iter()
        .flat_map(|p| p.shape.vertices(&p.bottom_left).into_iter())
        .collect();
    for yy in (0..=top_y).rev() {
        for xx in 1..=7 {
            if !seen[xx - 1] && vertices.contains(&(xx, yy)) {
                seen[xx - 1] = true;
                out[xx - 1] = yy;
            }
        }
        if seen.iter().all(|&s| s) {
            break;
        }
    }
    let lowest = *out.iter().min().unwrap();
    for o in &mut out {
        *o = *o - lowest;
    }
    out
}

pub fn task1() {
    let mut movements = vec![];
    for c in get_input(17).chars() {
        if c == '<' || c == '>' {
            movements.push(c.into());
            movements.push(Direction::Down);
        } else {
            break;
        }
    }

    let mut piece_idx = 0;
    let mut placed = vec![];
    let mut piece = Piece {
        bottom_left: (3, 4),
        shape: Shape::nth(0),
    };
    for movement in movements.into_iter().cycle() {
        if piece_idx == 0 {
            print_cave(&piece, &placed);
        }
        if piece.would_intersect(&placed, &movement) {
            if let Direction::Down = movement {
                // Change out the piece
                placed.push(piece);
                piece_idx += 1;
                let y = placed
                    .iter()
                    .map(|p| p.shape.top(p.bottom_left.1))
                    .max()
                    .unwrap()
                    + 4;
                piece = Piece {
                    bottom_left: (3, y),
                    shape: Shape::nth(piece_idx),
                };
                if piece_idx == 2022 {
                    break;
                }
            }
        } else {
            piece.bottom_left += movement;
        }
    }
    println!(
        "{}",
        placed
            .iter()
            .map(|p| p.shape.top(p.bottom_left.1))
            .max()
            .unwrap()
    );
}

pub fn task2() {
    let mut movements = vec![];
    for c in get_input(17).chars() {
        if c == '<' || c == '>' {
            movements.push(c.into());
            movements.push(Direction::Down);
        } else {
            break;
        }
    }

    let mut piece_idx = 0;
    let mut placed = vec![];
    let mut piece = Piece {
        bottom_left: (3, 4),
        shape: Shape::nth(0),
    };

    // We have to find a pattern! If we ever encounter a scenario where we have the same upper topology, with the same shape, at the same index,
    // Then we know for a fact that it will repeat
    let mut history: HashMap<(usize, Shape, [usize; 7]), (usize, usize)> = HashMap::new();
    let mut remaining_pieces = 0;
    let mut base_height = 0;
    let mut move_idx = 0;
    for (m_idx, movement) in movements.iter().enumerate().cycle() {
        if piece.would_intersect(&placed, movement) {
            if let Direction::Down = movement {
                // Record for posterity (or quit because we got it)
                let key = (m_idx, Shape::nth(piece_idx), get_topology(&placed));
                let current_height = placed
                    .iter()
                    .map(|p| p.shape.top(p.bottom_left.1))
                    .max()
                    .unwrap_or(0);
                if let Some((height, old_piece_idx)) = history.get(&key) {
                    let pieces_in_pattern = piece_idx - old_piece_idx;
                    let pattern_height = current_height - height;
                    let pieces_left = 1000000000000 - piece_idx;
                    let num_patterns = pieces_left / pieces_in_pattern;
                    remaining_pieces = pieces_left - (num_patterns * pieces_in_pattern);
                    base_height = pattern_height * num_patterns + current_height;
                    move_idx = m_idx;
                    break;
                } else {
                    history.insert(key, (current_height, piece_idx));
                }
                // Change out the piece
                placed.push(piece);
                piece_idx += 1;
                let y = placed
                    .iter()
                    .map(|p| p.shape.top(p.bottom_left.1))
                    .max()
                    .unwrap()
                    + 4;
                piece = Piece {
                    bottom_left: (3, y),
                    shape: Shape::nth(piece_idx),
                };
            }
        } else {
            piece.bottom_left += *movement;
        }
    }
    // Simulate the rest of the pieces (which weren't captured by the pattern). We're effectively replaying the "leftover" part of the pattern,
    // since 1 trillion doesn't (necessarily) fit precisely into the pattern.
    let base_idx = piece_idx;
    let height_before = placed
        .iter()
        .map(|p| p.shape.top(p.bottom_left.1))
        .max()
        .unwrap_or(0);
    for movement in movements.iter().cycle().skip(move_idx) {
        if piece.would_intersect(&placed, movement) {
            if let Direction::Down = movement {
                // Change out the piece
                if piece_idx == (remaining_pieces + base_idx) {
                    let current_height = placed
                        .iter()
                        .map(|p| p.shape.top(p.bottom_left.1))
                        .max()
                        .unwrap_or(0);
                    // Add the height from our pattern calculation (which includes the prelude), then add the
                    // height of the replayed blocks
                    println!("{}", base_height + current_height - height_before);
                    break;
                }
                placed.push(piece);
                piece_idx += 1;
                let y = placed
                    .iter()
                    .map(|p| p.shape.top(p.bottom_left.1))
                    .max()
                    .unwrap()
                    + 4;
                piece = Piece {
                    bottom_left: (3, y),
                    shape: Shape::nth(piece_idx),
                };
            }
        } else {
            piece.bottom_left += *movement;
        }
    }
    //1586627906921
}
