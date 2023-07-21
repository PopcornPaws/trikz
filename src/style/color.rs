use crate::svgutils::Value;

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

impl From<Color> for Value {
    fn from(color: Color) -> Value {
        match color {
            Color::Black => "black".into(),
            Color::Blue => "blue".into(),
            Color::Cyan => "cyan".into(),
            Color::Gray => "gray".into(),
            Color::Green => "green".into(),
            Color::Magenta => "magenta".into(),
            Color::Red => "red".into(),
            Color::White => "white".into(),
            Color::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b).into(),
        }
    }
}
