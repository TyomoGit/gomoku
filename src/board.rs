use std::fmt::{Debug, Display, Write};

use dyn_clone::DynClone;

use crate::{error::{GomokuError, Result}, point::Point, stone::Stone};

pub type Board = Vec<Vec<Option<Stone>>>;

pub const DEFAULT_BOARD_SIZE: usize = 15;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub trait GomokuBoard: DynClone {
    fn size(&self) -> usize;
    fn board(&self) -> &Board;
    fn board_mut(&mut self) -> &mut Board;

    fn get_at(&self, x: usize, y: usize) -> Option<Stone>;
    fn in_range(&self, x: usize, y: usize) -> bool;
    fn count(&self, player: Stone) -> usize;

    fn is_full(&self) -> bool;

    fn put_stone(&mut self, x: usize, y: usize, player: Stone) -> Result<()>;
    fn check_win_at(&self, x: usize, y: usize) -> bool;
}

impl Debug for dyn GomokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board();
        for row in board {
            for cell in row {
                match cell {
                    Some(Stone::Black) => write!(f, "⚫︎")?,
                    Some(Stone::White) => write!(f, "⚪︎")?,
                    None => write!(f, "[]")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for dyn GomokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = format!("{:?}", self);
        let mut result = String::new();

        write!(result, "   | ")?;

        for i in 0..self.board().len() {
            let alpha = (b'A' + i as u8) as char;
            write!(result, "{: ^2}", alpha)?;
        }

        writeln!(result)?;

        writeln!(result, "---+-{}", "--".repeat(self.board().len()))?;

        for (i, row) in board.lines().enumerate() {
            writeln!(result, "{: ^2} | {}", i + 1, row)?;
        }

        write!(f, "{}", result)
    }
}

#[derive(Clone)]
pub struct ArrayBasedBoard {
    board: Board,
    stone_count: usize,
}

impl Default for ArrayBasedBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayBasedBoard {
    pub fn new() -> Self {
        Self::with_size(DEFAULT_BOARD_SIZE)
    }

    pub fn with_size(size: usize) -> Self {
        if size < 5 {
            panic!("Board size must be at least 5");
        }

        Self {
            board: vec![vec![None; size]; size],
            stone_count: 0,
        }
    }
}

impl GomokuBoard for ArrayBasedBoard {
    fn size(&self) -> usize {
        self.board.len()
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    fn get_at(&self, x: usize, y: usize) -> Option<Stone> {
        self.board.get(y).and_then(|row| row.get(x).copied()).flatten()
    }

    fn in_range(&self, x: usize, y: usize) -> bool {
        (0..self.size()).contains(&x) && (0..self.size()).contains(&y)
    }

    #[inline]
    fn count(&self, player: Stone) -> usize {
        self.stone_count
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&x| x.is_some()))
    }

    fn put_stone(&mut self, x: usize, y: usize, player: Stone) -> Result<()> {
        if self.get_at(x, y).is_some() {
            return Err(GomokuError::StoneAlreadyPlaced);
        }

        if !self.in_range(x, y) {
            return Err(GomokuError::IndexOutOfBound);
        }

        self.board_mut()[y][x] = Some(player);
        self.stone_count += 1;

        if self.check_win_at(x, y) {
            Err(GomokuError::GameOverWithWinner(player))
        } else if self.is_full() {
            Err(GomokuError::GameOverWithDraw)
        } else {
            Ok(())
        }
    }
    
    fn check_win_at(&self, x: usize, y: usize) -> bool {
        let Some(player) = self.get_at(x, y) else {
            return false;
        };

        for d in DIRECTIONS {
            let mut stack: Vec<Point> = Vec::new();

            let mut x = x as i32;
            let mut y = y as i32;

            if !self.in_range(x as usize, y as usize) {
                continue;
            }

            for _ in 0..5 {
                if self.get_at(x as usize, y as usize) != Some(player) {
                    break;
                }

                stack.push(Point::new(x as usize, y as usize));
                x += d.0;
                y += d.1;
            }

            if stack.len() >= 5 {
                return true;
            }
        }

        false
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_5() {
        let mut board = ArrayBasedBoard::new();

        for i in 0..4 {
            let result = board.put_stone(i as usize, 0, Stone::Black);
            assert_eq!(result, Ok(()));
        }

        let result = board.put_stone(4, 0, Stone::Black);
        assert_eq!(result, Err(GomokuError::GameOverWithWinner(Stone::Black)));
    }
}
