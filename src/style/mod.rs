mod color;
mod stroke;

pub use color::Color;
pub use stroke::Stroke;

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Default)]
pub struct Style {
    pub fill: Color,
    pub stroke: Stroke,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(self, fill: Color) -> Self {
        Self {
            fill,
            stroke: self.stroke,
        }
    }

    pub fn stroke(self, stroke: Stroke) -> Self {
        Self {
            fill: self.fill,
            stroke,
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "fill: {}; {}",
            self.fill,
            self.stroke,
        )
    }
}
