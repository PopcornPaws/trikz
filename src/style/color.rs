use crate::svg::Value;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Gray,
    Green,
    Magenta,
    Red,
    White,
    Rgb(u8, u8, u8),
}

impl Into<Value> for Color {
    fn into(self) -> Value {
        match self {
            Self::Black => "black".into(),
            Self::Blue => "blue".into(),
            Self::Cyan => "cyan".into(),
            Self::Gray => "gray".into(),
            Self::Green => "green".into(),
            Self::Magenta => "magenta".into(),
            Self::Red => "red".into(),
            Self::White => "white".into(),
            Self::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b).into(),
        }
    }
}
