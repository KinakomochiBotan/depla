use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use anyhow::{
    Result,
    ensure
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Position {
    value: u32
}

impl Position {

    #[inline]
    pub fn of(value: u32) -> Result<Self> {
        ensure!(value < 64, "a position must be less than 64, but it was {}", value);
        Result::Ok(unsafe { Self::of_unchecked(value) })
    }

    #[inline]
    pub const unsafe fn of_unchecked(value: u32) -> Self {
        Self { value }
    }

    #[inline]
    pub fn at(row: u32, column: u32) -> Result<Self> {
        ensure!(row < 8 && column < 8, "a row and a column must be less than 8, but they were {} and {}", row, column);
        Result::Ok(unsafe { Self::at_unchecked(row, column) })
    }

    #[inline]
    pub const unsafe fn at_unchecked(row: u32, column: u32) -> Self {
        Self { value: 8 * row + column }
    }

    #[inline]
    pub const fn value(self) -> u32 {
        self.value
    }

    #[inline]
    pub const fn row(self) -> u32 {
        self.value / 8
    }

    #[inline]
    pub const fn column(self) -> u32 {
        self.value % 8
    }

    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..64).map(|position| unsafe { Self::of_unchecked(position) })
    }

}

impl Display for Position {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.row(), self.column())
    }
}
