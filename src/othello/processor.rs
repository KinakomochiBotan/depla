use super::{
    BoardIndex,
    Board
};

use anyhow::Result;

#[derive(Clone, Copy)]
pub enum PlayerType {
    Black,
    White
}

impl PlayerType {
    #[inline]
    fn next(&mut self) {
        *self = match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

pub trait Players {
    fn get_move(&mut self, player_type: PlayerType, board: Board) -> BoardIndex;
}

#[inline]
pub fn run<P: Players>(players: &mut P) -> Result<()> {
    let mut game = Board::new();
    let mut player_type = PlayerType::Black;
    let mut will_pass = false;

    loop {

        if game.legal().value() == 0 {
            if will_pass {
                break;
            } else {
                will_pass = true;
                game.pass();
                player_type.next();
                continue;
            }
        }

        game.put(players.get_move(player_type, game))?;
        will_pass = game.legal().value() == 0;
        game.pass();
        player_type.next();
    }

    return Result::Ok(());
}

pub trait Player {
    fn get_move(&mut self, player_type: PlayerType, board: Board) -> BoardIndex;
}

pub struct DefaultPlayers<B, W> {
    black: B,
    white: W
}

impl<B, W> DefaultPlayers<B, W> {
    #[inline]
    pub fn new(black: B, white: W) -> Self {
        Self {
            black,
            white
        }
    }
}

impl<B: Player, W: Player> Players for DefaultPlayers<B, W> {
    #[inline]
    fn get_move(&mut self, player_type: PlayerType, board: Board) -> BoardIndex {
        match player_type {
            PlayerType::Black => self.black.get_move(player_type, board),
            PlayerType::White => self.white.get_move(player_type, board)
        }
    }
}
