use crate::players::Dataset as PlayersDataset;

#[derive(Clone, Debug)]
pub struct Dataset {
    data: Vec<PlayersDataset>,
    length: usize
}

impl Dataset {

    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            length: 0
        }
    }

    #[inline]
    pub fn push(&mut self, data: PlayersDataset) {
        self.length += data.len();
        self.data.push(data);
    }

    #[inline]
    pub fn data(self) -> Vec<PlayersDataset> {
        self.data
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

}
