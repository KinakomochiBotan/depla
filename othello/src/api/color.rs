use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Black,
    White
}

impl Color {
    #[inline]
    pub fn flip(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

impl Display for Color {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
