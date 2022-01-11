mod dataset;

pub use self::dataset::Dataset;

use ndarray::{
    Array2,
    Array3
};

use othello::{
    Index,
    Data as OthelloData
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Data {
    player: OthelloData,
    opponent: OthelloData,
    position: OthelloData
}

impl Data {

    #[inline]
    pub fn new(player: OthelloData, opponent: OthelloData, position: OthelloData) -> Self {
        Self {
            player,
            opponent,
            position
        }
    }

    #[inline]
    pub fn flip_vertical(&self) -> Self {
        let flip_vertical = |data: OthelloData| OthelloData::of(data.value().swap_bytes());
        return Self::new(flip_vertical(self.player), flip_vertical(self.opponent), flip_vertical(self.position));
    }

    #[inline]
    pub fn rotate180(&self) -> Self {
        let rotate180 = |data: OthelloData| OthelloData::of(data.value().reverse_bits());
        return Self::new(rotate180(self.player), rotate180(self.opponent), rotate180(self.position));
    }

    #[inline]
    pub fn flip_diagonal(&self) -> Self {

        let flip_diagonal = |data: OthelloData| {
            let mut result = data.value();

            for (n, mask) in [
                (28, 0x0f0f0f0f00000000),
                (14, 0x3333000033330000),
                (07, 0x5500550055005500)
            ] {
                let mask = mask & (result ^ (result << n));
                result ^= mask ^ (mask >> n);
            }

            return OthelloData::of(result);
        };

        return Self::new(flip_diagonal(self.player), flip_diagonal(self.opponent), flip_diagonal(self.position));
    }

    #[inline]
    pub fn to(self) -> (Array3<f32>, Array2<f32>) {
        let mut data = Array3::<f32>::zeros((2, 8, 8));
        let mut label = Array2::<f32>::zeros((8, 8));

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if self.player.is_set(index) {
                    data[(0, row, column)] = 1.0;
                }

                if self.opponent.is_set(index) {
                    data[(1, row, column)] = 1.0;
                }

                if self.position.is_set(index) {
                    label[(row, column)] = 1.0;
                }

            }
        }

        return (data, label);
    }

}
