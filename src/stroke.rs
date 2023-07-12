use crate::Scalar;
use crate::color::Color;

pub struct Stroke {
    pub color: Color,
    pub opacity: Scalar,
    pub width: usize,
    pub style: Style,
}

// TODO stroke-dasharray
pub enum Style {
    Dashed,
    Dotted,
    Dashdotted,
    Solid,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::Black,
            opacity: 1.0,
            width: 5,
            style: Style::Solid,
        }
    }
}
