mod color;
mod font;
mod stroke;

pub use color::Color;
pub use font::Font;
pub use stroke::Stroke;

use crate::transform::{keys, svg, WriteAttribute};

#[derive(Clone, Debug)]
pub struct Style<T> {
    pub fill: Option<Color>,
    pub ty: Option<T>,
}

impl<T> Style<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(self, fill: Color) -> Self {
        Self {
            fill: Some(fill),
            ty: self.ty,
        }
    }
}

impl Style<Stroke> {
    pub fn stroke(self, stroke: Stroke) -> Self {
        Self {
            fill: self.fill,
            ty: Some(stroke),
        }
    }
}

impl Style<Font> {
    pub fn font(self, font: Font) -> Self {
        Self {
            fill: self.fill,
            ty: Some(font),
        }
    }
}

impl<T: WriteAttribute> WriteAttribute for Style<T> {
    fn write(&self, attributes: &mut svg::Attributes) {
        if let Some(fill) = self.fill {
            attributes.insert(keys::FILL.into(), fill.into());
        }
        if let Some(ref ty) = self.ty {
            ty.write(attributes);
        }
    }
}

impl<T> Default for Style<T> {
    fn default() -> Self {
        Self {
            fill: None,
            ty: None,
        }
    }
}
