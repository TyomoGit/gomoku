use std::fmt::Display;

use crate::{board::{ArrayBasedBoard, GomokuBoard}, error::Result, stone:: Stone};

pub struct SimpleGomokuGame {
    board: Box<dyn GomokuBoard>,
    turn: Stone,
}

impl SimpleGomokuGame {
    pub fn new() -> Self {
        Self {
            board: Box::new(ArrayBasedBoard::new()),
            turn: Stone::Black,
        }
    }

    pub fn put_stone(&mut self, x: usize, y: usize) -> Result<()> {
        let result = self.board.put_stone(x, y, self.turn);
        self.take_turn();
        result
    }

    #[inline]
    pub fn board(&self) -> &dyn GomokuBoard {
        self.board.as_ref()
    }

    #[inline]
    pub fn board_mut(&mut self) -> &mut dyn GomokuBoard {
        self.board.as_mut()
    }

    #[inline]
    pub fn take_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    #[inline]
    pub fn turn(&self) -> Stone {
        self.turn
    }

    #[inline]
    pub fn set_turn(&mut self, turn: Stone) {
        self.turn = turn;
    }
}

impl Default for SimpleGomokuGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SimpleGomokuGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::GomokuError;

    use super::*;

    #[test]
    fn test1() {
        let mut game = SimpleGomokuGame::new();

        for i in 0..4 {
            game.put_stone(i, 0).unwrap(); // black
            game.put_stone(i, 1).unwrap(); // white
        }

        // black
        game.put_stone(14, 0).unwrap();

        // white
        let result = game.put_stone(4, 1);
        assert_eq!(result, Err(GomokuError::GameOverWithWinner(Stone::White)));
    }
}
