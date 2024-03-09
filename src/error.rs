use crate::stone::Stone;

pub type Result<T> = std::result::Result<T, GomokuError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GomokuError {
    StoneAlreadyPlaced,
    InvalidMove,
    IndexOutOfBound,

    GameOverWithWinner(Stone),
    GameOverWithDraw,
}
