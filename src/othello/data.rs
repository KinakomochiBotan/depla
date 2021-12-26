use super::BoardIndex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardData {
    value: u64
}

impl BoardData {

    #[inline]
    pub const fn of(data: u64) -> Self {
        Self {
            value: data
        }
    }

    #[inline]
    pub const fn new() -> Self {
        Self::of(0)
    }

    #[inline]
    pub const fn set(self, index: BoardIndex) -> Self {
        Self::of(self.value | (1 << index.value()))
    }

    #[inline]
    pub const fn is_set(self, index: BoardIndex) -> bool {
        self.value & (1 << index.value()) != 0
    }

    #[inline]
    pub fn count(self) -> u32 {
        self.value.count_ones()
    }

    #[inline]
    pub const fn value(self) -> u64 {
        self.value
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let index1 = BoardIndex::of(45).unwrap();
        let index2 = BoardIndex::at(2, 7).unwrap();
        let index3 = BoardIndex::of(18).unwrap();
        let data = BoardData::new().set(index1).set(index2).set(index3);
        assert_eq!(data.value, 0x0000200000840000);
    }

    #[test]
    fn test_is_set() {
        let index1 = BoardIndex::at(6, 7).unwrap();
        let index2 = BoardIndex::at(4, 3).unwrap();
        let index3 = BoardIndex::at(4, 3).unwrap();
        let index4 = BoardIndex::at(1, 7).unwrap();
        let data = BoardData::new().set(index1).set(index2);
        assert!(data.is_set(index3));
        assert!(!data.is_set(index4));
    }

    #[test]
    fn test_count() {
        let index1 = BoardIndex::at(0, 0).unwrap();
        let index2 = BoardIndex::at(3, 2).unwrap();
        let index3 = BoardIndex::at(7, 3).unwrap();
        let data = BoardData::new().set(index1).set(index2).set(index3);
        assert_eq!(data.count(), 3);
    }

}
