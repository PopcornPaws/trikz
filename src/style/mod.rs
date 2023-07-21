mod color;
mod font;
mod stroke;

pub use color::Color;
pub use font::Font;
pub use stroke::Stroke;

use crate::svgutils::{keys, Attributes, ToAttributes};

#[derive(Clone, Debug)]
pub struct Style<T> {
    pub fill: Option<Color>, // same for text-based stuff (although color would also be valid there)
    pub repr: Option<T>,
}

impl<T> Style<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(self, fill: Color) -> Self {
        Self {
            fill: Some(fill),
            repr: self.repr,
        }
    }
}

impl Style<Stroke> {
    pub fn stroke(self, stroke: Stroke) -> Self {
        Self {
            fill: self.fill,
            repr: Some(stroke),
        }
    }
}

impl Style<Font> {
    pub fn font(self, font: Font) -> Self {
        Self {
            fill: self.fill,
            repr: Some(font),
        }
    }
}

impl<T: ToAttributes> ToAttributes for Style<T> {
    fn to_attributes(&self, attributes: &mut Attributes) {
        if let Some(color) = self.fill {
            attributes.insert(keys::FILL.into(), color.into());
        }
        if let Some(repr) = self.repr.as_ref() {
            repr.to_attributes(attributes);
        }
    }
}

impl<T> Default for Style<T> {
    fn default() -> Self {
        Self {
            fill: None,
            repr: None,
        }
    }
}
