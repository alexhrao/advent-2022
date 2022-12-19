use std::{ops::{Index, IndexMut}, fmt::Display};

use crate::utils::get_input;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    pub is_end: bool,
    pub elev: u8,
    pub dir: Option<Direction>,
}
#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    pub dims: (usize, usize),
}

impl Board {
    pub fn new() -> Board {
        let tiles: Vec<Vec<Tile>> = get_input(12)
        .lines()
        .map(|l| l.bytes().map(|c| {
            let is_end = c == 'E' as u8;
            Tile { elev: if is_end { 'z' as u8 } else { c }, is_end, dir: None, }
        }).collect())
        .collect(); 
        let dims = (tiles.len(), tiles[0].len());
        Board { tiles, dims, }
    }

    pub fn explore(mut self) -> Option<(Board, Vec<(usize, usize)>)> {
        // Find the start, convert to an a, then engage
        let mut start = (0, 0);
        for r in 0..self.dims.0 {
            for c in 0..self.dims.1 {
                if self.tiles[r][c].elev == 'S' as u8 {
                    start = (r, c);
                    self.tiles[r][c].elev = 'a' as u8;
                }
            }
        }
        explore(start, self)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Tile;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        &self.tiles[r][c]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        &mut self.tiles[r][c]
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut tiles = self.tiles.clone();
        for r in 0..tiles.len() {
            tiles[r] = self.tiles[r].clone();
        }
        Board { tiles, dims: self.dims, }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for col in row {
                f.write_str(match col.dir {
                    Some(Direction::Down) => "v",
                    Some(Direction::Up) => "^",
                    Some(Direction::Left) => "<",
                    Some(Direction::Right) => ">",
                    None => ".",
                })?;
            }
            f.write_fmt(format_args!("\n",))?;
        }
        Ok(())
    }
}

fn explore(loc: (usize, usize), board: Board) -> Option<(Board, Vec<(usize, usize)>)> {
    let (r, c) = loc;
    let tile = board[loc];
    
    if tile.is_end {
        return Some((board, vec![]));
    }
    let mut possibilities = vec![];
    // See if I can go up:
    if r > 0 && board[(r - 1, c)].dir.is_none() && board[(r - 1, c)].elev <= tile.elev + 1 {
        let mut up_board = board.clone();
        up_board[loc].dir = Some(Direction::Up);
        if let Some((b, mut moves)) = explore((r - 1, c), up_board) {
            moves.push(loc);
            possibilities.push((b, moves));
        }
    }
    // See if I can go down:
    if r < (board.dims.0 - 1) && board[(r + 1, c)].dir.is_none() && board[(r + 1, c)].elev <= (tile.elev + 1) {
        let mut down_board = board.clone();
        down_board[loc].dir = Some(Direction::Down);
        if let Some((b, mut moves)) = explore((r + 1, c), down_board) {
            moves.push(loc);
            possibilities.push((b, moves));
        }
    }
    // See if I can go left:
    if c > 0 && board[(r, c - 1)].dir.is_none() && board[(r, c - 1)].elev <= (tile.elev + 1) {
        let mut left_board = board.clone();
        left_board[loc].dir = Some(Direction::Left);
        if let Some((b, mut moves)) = explore((r, c - 1), left_board) {
            moves.push(loc);
            possibilities.push((b, moves));
        }
    }
    // See if I can go right:
    if c < (board.dims.1 - 1) && board[(r, c + 1)].dir.is_none() && board[(r, c + 1)].elev <= (tile.elev + 1) {
        let mut right_board = board.clone();
        right_board[loc].dir = Some(Direction::Right);
        if let Some((b, mut moves)) = explore((r, c + 1), right_board) {
            moves.push(loc);
            possibilities.push((b, moves));
        }
    }

    possibilities.iter().min_by_key(|p| p.1.len()).cloned()
}

pub fn task1() {
    if let Some((_, m)) = Board::new().explore() {
        println!("{}", m.len());
    }
    //let path_lens = vec![];
    todo!();
}

pub fn task2() {
    todo!();
}