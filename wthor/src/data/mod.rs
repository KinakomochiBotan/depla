mod set;

pub use self::set::*;

use ndarray::Array3;

use othello::api::{
    Position,
    Data as OthelloData
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Data {
    player: OthelloData,
    opponent: OthelloData,
    position: Position
}

impl Data {

    #[inline]
    pub fn new(player: OthelloData, opponent: OthelloData, position: Position) -> Self {
        Self { player, opponent, position }
    }

    #[inline]
    fn flip_vertical(self) -> Self {
        let calc_data = |data: OthelloData| OthelloData::of(crate::bit::flip_vertical(data.value()));
        let calc_position = |position: Position| unsafe { Position::at_unchecked(7 - position.row(), position.column()) };
        Self::new(calc_data(self.player), calc_data(self.opponent), calc_position(self.position))
    }

    #[inline]
    fn rotate180(self) -> Self {
        let calc_data = |data: OthelloData| OthelloData::of(crate::bit::rotate180(data.value()));
        let calc_position = |position: Position| unsafe { Position::at_unchecked(7 - position.row(), 7 - position.column()) };
        Self::new(calc_data(self.player), calc_data(self.opponent), calc_position(self.position))
    }

    #[inline]
    fn flip_diagonal(self) -> Self {
        let calc_data = |data: OthelloData| OthelloData::of(crate::bit::flip_diagonal(data.value()));
        let clac_position = |position: Position| unsafe { Position::at_unchecked(position.column(), position.row()) };
        Self::new(calc_data(self.player), calc_data(self.opponent), clac_position(self.position))
    }

    #[inline]
    pub fn augment(self) -> [Self; 8] {
        let mut result = [self; 8];
        result[4] = result[4].flip_diagonal();
        result[5] = result[4];
        result[6] = result[4];
        result[7] = result[4];
        result[2] = result[2].rotate180();
        result[3] = result[2];
        result[6] = result[6].rotate180();
        result[7] = result[6];
        result[1] = result[1].flip_vertical();
        result[3] = result[3].flip_vertical();
        result[5] = result[5].flip_vertical();
        result[7] = result[7].flip_vertical();
        result
    }

    #[inline]
    pub fn into(self) -> (Array3<f32>, u32) {
        let mut data = Array3::<f32>::zeros((2, 8, 8));

        Position::iter().for_each(|position| {
            if self.player.is_set(position) { data[(0, position.row() as usize, position.column() as usize)] = 1.0; }
            if self.opponent.is_set(position) { data[(1, position.row() as usize, position.column() as usize)] = 1.0; }
        });

        (data, self.position.value())
    }

}
