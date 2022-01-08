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
    Py,
    PyAny
};

use numpy::IntoPyArray;

pub struct TrainData {
    data: Array3<f32>,
    label: Array2<f32>
}

impl TrainData {
    #[inline]
    pub fn new(player: Data, opponent: Data, position: Data) -> Self {
        let mut data = Array3::zeros((2, 8, 8));
        let mut label = Array2::zeros((8, 8));

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if player.is_set(index) {
                    data[(0, row, column)] = 1.0;
                }

                if opponent.is_set(index) {
                    data[(1, row, column)] = 1.0;
                }

                if position.is_set(index) {
                    label[(row, column)] = 1.0;
                }

            }
        }

        return Self {
            data,
            label
        }

    }
}

impl IntoPy<Py<PyAny>> for TrainData {
    #[inline]
    fn into_py(self, py: Python) -> Py<PyAny> {
        (self.data.into_pyarray(py), self.label.into_pyarray(py)).into_py(py)
    }
}
