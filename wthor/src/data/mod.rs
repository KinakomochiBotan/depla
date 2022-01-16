mod set;

pub use self::set::*;

use ndarray::Array3;

use othello::game::{
    Position,
    Data as OthelloData
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Data {
    player: OthelloData,
    opponent: OthelloData,
    position: Position
}

impl Data {

    #[inline]
    pub fn new(player: OthelloData, opponent: OthelloData, position: Position) -> Self {
        Self {
            player,
            opponent,
            position
        }
    }

    #[inline]
    pub fn flip_vertical(&self) -> Self {
        let calc_data = |data: OthelloData| OthelloData::of(data.value().swap_bytes());
        let calc_position = |position: Position| unsafe { Position::at_unchecked(7 - position.row(), position.column()) };
        Self::new(calc_data(self.player), calc_data(self.opponent), calc_position(self.position))
    }

    #[inline]
    pub fn rotate180(&self) -> Self {
        let calc_data = |data: OthelloData| OthelloData::of(data.value().reverse_bits());
        let calc_position = |position: Position| unsafe { Position::at_unchecked(7 - position.row(), 7 - position.column()) };
        Self::new(calc_data(self.player), calc_data(self.opponent), calc_position(self.position))
    }

    #[inline]
    pub fn flip_diagonal(&self) -> Self {

        #[inline]
        const fn flip_diagonal(data: u64) -> u64 {
            let mut result = data;

            macro_rules! calc {
                ($n:literal, $m:literal) => {
                    let mask = $m & (result ^ (result << $n));
                    result ^= mask ^ (mask >> $n);
                };
            }

            calc!(28, 0x0f0f0f0f00000000);
            calc!(14, 0x3333000033330000);
            calc!(07, 0x5500550055005500);
            return result;
        }

        let calc_data = |data: OthelloData| OthelloData::of(flip_diagonal(data.value()));
        let clac_position = |position: Position| unsafe { Position::at_unchecked(position.column(), position.row()) };
        Self::new(calc_data(self.player), calc_data(self.opponent), clac_position(self.position))
    }

    #[inline]
    pub fn to(self) -> (Array3<f32>, u32) {
        let mut data = Array3::<f32>::zeros((2, 8, 8));

        Position::iter().for_each(|position| {
            if self.player.is_set(position) { data[(0, position.row() as usize, position.column() as usize)] = 1.0; }
            if self.opponent.is_set(position) { data[(1, position.row() as usize, position.column() as usize)] = 1.0; }
        });

        (data, self.position.value() as u32)
    }

}
