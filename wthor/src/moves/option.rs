use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LoadOption {
    win: bool,
    draw: bool,
    lose: bool
}

impl LoadOption {

    #[inline]
    pub fn new(win: bool, draw: bool, lose: bool) -> Self {
        Self { win, draw, lose }
    }

    #[inline]
    pub fn win(self) -> bool {
        self.win
    }

    #[inline]
    pub fn draw(self) -> bool {
        self.draw
    }

    #[inline]
    pub fn lose(self) -> bool {
        self.lose
    }

    #[inline]
    pub fn check(self) -> bool {
        self.win || self.draw || self.lose
    }

}

impl Display for LoadOption {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(win: {}, draw: {}, lose: {})", self.win, self.draw, self.lose)
    }
}
