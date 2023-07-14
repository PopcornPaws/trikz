use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Gray,
    Green,
    Magenta,
    None,
    Red,
    White,
    Rgb(u8, u8, u8),
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Black => write!(f, "black"),
            Self::Blue => write!(f, "blue"),
            Self::Cyan => write!(f, "cyan"),
            Self::Gray => write!(f, "gray"),
            Self::Green => write!(f, "green"),
            Self::Magenta => write!(f, "magenta"),
            Self::None => write!(f, "none"),
            Self::Red => write!(f, "red"),
            Self::White => write!(f, "white"),
            Self::Rgb(r, g, b) => write!(f, "#{:02X}{:02X}{:02X}", r, g, b),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::None
    }
}
