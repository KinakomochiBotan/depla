use ndarray::{
    Array2,
    Array3
};

use othello::{
    Index,
    Data
};

use pyo3::{
    Python,
    IntoPy,
    PyObject
};

use numpy::IntoPyArray;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrainData {
    player: Data,
    opponent: Data,
    position: Data
}

impl TrainData {

    #[inline]
    pub fn new(player: Data, opponent: Data, position: Data) -> Self {
        Self {
            player,
            opponent,
            position
        }
    }

    #[inline]
    pub fn flip_vertical(&self) -> Self {
        let flip_vertical = |data: Data| Data::of(data.value().swap_bytes());
        return Self::new(flip_vertical(self.player), flip_vertical(self.opponent), flip_vertical(self.position));
    }

    #[inline]
    pub fn rotate180(&self) -> Self {
        let rotate180 = |data: Data| Data::of(data.value().reverse_bits());
        return Self::new(rotate180(self.player), rotate180(self.opponent), rotate180(self.position));
    }

    #[inline]
    pub fn flip_diagonal(&self) -> Self {

        let flip_diagonal = |data: Data| {
            let mut result = data.value();

            for (n, mask) in [
                (28, 0x0f0f0f0f00000000),
                (14, 0x3333000033330000),
                (07, 0x5500550055005500)
            ] {
                let mask = mask & (result ^ (result << n));
                result ^= mask ^ (mask >> n);
            }

            return Data::of(result);
        };

        return Self::new(flip_diagonal(self.player), flip_diagonal(self.opponent), flip_diagonal(self.position));
    }

}

impl IntoPy<PyObject> for TrainData {
    #[inline]
    fn into_py(self, py: Python) -> PyObject {
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

        return (data.into_pyarray(py), label.into_pyarray(py)).into_py(py);
    }
}
