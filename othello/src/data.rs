use crate::Index;

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Data {
    value: u64
}

impl Data {

    #[inline]
    pub const fn of(value: u64) -> Self {
        Self {
            value
        }
    }

    #[inline]
    pub const fn set(self, index: Index) -> Self {
        Self::of(self.value | (1 << index.value()))
    }

    #[inline]
    pub const fn is_set(self, index: Index) -> bool {
        self.value & Self::of(0).set(index).value != 0
    }

    #[inline]
    pub const fn value(self) -> u64 {
        self.value
    }

}

impl Display for Data {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:b}", self.value)
    }
}
