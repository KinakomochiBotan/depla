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
    pub fn of(value: usize) -> Result<Self> {
        match value < 64 {
            true => Result::Ok(Self {
                value
            }),
            false => Result::Err(anyhow::anyhow!("an index must be less than 64, but it was {}", value))
        }
    }

    #[inline]
    pub fn at(row: usize, column: usize) -> Result<Self> {
        match row < 8 && column < 8 {
            true => Result::Ok(Self {
                value: 8 * row + column
            }),
            false => Result::Err(anyhow::anyhow!("a row and a column must be less than 8, but it was {} and {}", row, column))
        }
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
