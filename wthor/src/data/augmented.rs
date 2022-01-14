use super::Data;
use ndarray::Array3;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AugmentedData {
    data: [Data; 8]
}

impl AugmentedData {

    #[inline]
    pub fn of(data: Data) -> Self {
        let mut data = [data; 8];
        data[4] = data[4].flip_diagonal();
        data[5] = data[4];
        data[6] = data[4];
        data[7] = data[4];
        data[2] = data[2].rotate180();
        data[3] = data[2];
        data[6] = data[6].rotate180();
        data[7] = data[6];
        data[1] = data[1].flip_vertical();
        data[3] = data[3].flip_vertical();
        data[5] = data[5].flip_vertical();
        data[7] = data[7].flip_vertical();
        data.sort();
        Self { data }
    }

    #[inline]
    pub fn to(self) -> [(Array3<f32>, u32); 8] {
        self.data.map(|data| data.to())
    }

}
