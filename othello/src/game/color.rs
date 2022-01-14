use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiscColor {
    Black,
    White
}

impl DiscColor {
    #[inline]
    pub fn flip(&mut self) {
        *self = match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

impl Display for DiscColor {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
