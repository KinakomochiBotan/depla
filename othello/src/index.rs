use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Index {
    value: usize
}

impl Index {

    #[inline]
    fn new(value: usize) -> Self {
        Self {
            value
        }
    }

    #[inline]
    pub fn of(value: usize) -> Result<Self> {
        anyhow::ensure!(value < 64, "an index must be less than 64, but it was {}", value);
        return Result::Ok(Self::new(value));
    }

    #[inline]
    pub fn at(row: usize, column: usize) -> Result<Self> {
        anyhow::ensure!(row < 8 && column < 8, "a row and a column must be less than 8, but it was {} and {}", row, column);
        return Result::Ok(Self::new(8 * row + column));
    }

    #[inline]
    pub const fn value(self) -> usize {
        self.value
    }

    #[inline]
    pub const fn row(self) -> usize {
        self.value / 8
    }

    #[inline]
    pub const fn column(self) -> usize {
        self.value % 8
    }

}

impl Display for Index {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.row(), self.column())
    }
}
