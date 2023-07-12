use crate::Scalar;
use crate::color::Color;

pub struct Stroke {
    color: Color,
    opacity: Scalar,
    style: Style,
    width: Scalar,
}

// TODO stroke-dasharray
pub enum Style {
    Dashed,
    Dotted,
    Dashdotted,
    Solid,
}
