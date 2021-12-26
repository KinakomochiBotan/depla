mod index;
mod data;
mod board;
mod processor;

pub use self::{
    index::BoardIndex,
    data::BoardData,
    board::Board,
    processor::{
        PlayerType,
        Players,
        Player,
        DefaultPlayers,
        run
    }
};
