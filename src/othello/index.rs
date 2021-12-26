use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardIndex {
    value: usize
}

impl BoardIndex {

    #[inline]
    pub fn of(index: usize) -> Result<Self> {
        match index < 64 {
            true => Result::Ok(Self {
                value: index
            }),
            false => Result::Err(anyhow::anyhow!("an index must be less than 64, but it was {}", index))
        }
    }

    #[inline]
    pub fn at(row: usize, column: usize) -> Result<Self> {
        Self::of(8 * row + column)
    }

    #[inline]
    pub const fn value(self) -> usize {
        self.value
    }

}
