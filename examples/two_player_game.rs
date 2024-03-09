use std::io::stdin;

use gomoku::{game::SimpleGomokuGame, stone::Stone};

fn main() {
    let mut game = SimpleGomokuGame::new();

    loop {
        println!("{}", game);
        println!(
            "{}'s turn",
            if game.turn() == Stone::Black {
                "⚫︎"
            } else {
                "⚪︎"
            }
        );

        let Some((x, y)) = read_input() else {
            continue;
        };

        let Err(error) = game.put_stone(x, y) else {
            continue;
        };

        match error {
            gomoku::error::GomokuError::StoneAlreadyPlaced
            | gomoku::error::GomokuError::InvalidMove
            | gomoku::error::GomokuError::IndexOutOfBound => {
                println!("{:?}", error);
            }
            gomoku::error::GomokuError::GameOverWithWinner(winner) => {
                println!("{:?} wins!", winner);
                break;
            }
            gomoku::error::GomokuError::GameOverWithDraw => {
                println!("Draw.");
                break;
            }
        }
    }
}

fn read_input() -> Option<(usize, usize)> {
    let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        let mut split = buff.trim().split(' ');

        let Some(x) = split.next() else {
            return None;
        };
        let Some(y) = split.next() else {
            return None;
        };

        let Some(x) = x.chars().next() else {
            return None;
        };
        let Ok(y) = y.parse::<usize>() else {
            return None;
        };
        let y = y - 1;

        let x = if x.is_ascii_uppercase() {
            x as usize - 'A' as usize
        } else {
            x as usize - 'a' as usize
        };

        Some((x, y))
}
