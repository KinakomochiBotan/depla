use crate::data::Data;

#[derive(Clone, Debug)]
pub struct Dataset {
    data: Vec<Data>
}

impl Dataset {

    #[inline]
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    #[inline]
    pub fn push(&mut self, data: Data) {
        self.data.reserve(8);
        self.push8(data);
    }

    #[inline]
    fn push1(&mut self, data: Data) {
        self.data.push(data);
    }

    #[inline]
    fn push2(&mut self, data: Data) {
        self.push1(data);
        self.push1(data.flip_vertical());
    }

    #[inline]
    fn push4(&mut self, data: Data) {
        self.push2(data);
        self.push2(data.rotate180());
    }

    #[inline]
    fn push8(&mut self, data: Data) {
        self.push4(data);
        self.push4(data.flip_diagonal());
    }

    #[inline]
    pub fn data(self) -> Vec<Data> {
        self.data
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

}
