use super::Position;

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Data {
    value: u64
}

impl Data {

    #[inline]
    pub const fn of(value: u64) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn is_set(self, position: Position) -> bool {
        self.value & position.into_data().value != 0
    }

    #[inline]
    pub const fn count(self) -> u32 {
        crate::bit::count(self.value)
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
