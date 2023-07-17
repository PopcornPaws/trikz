mod color;
mod font;
mod stroke;

pub use color::Color;
pub use font::Font;
pub use stroke::Stroke;
pub use transform::Transform;

use crate::transform::{keys, svg, WriteAttributes};

#[derive(Clone, Debug, Default)]
pub struct Style<T: Default> {
    pub fill: Option<Color>,
    pub transform: Transform,
    pub ty: Option<T>,
}

impl<T> Style<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(self, fill: Color) -> Self {
        Self {
            fill: Some(fill),
            transform: self.transform,
            ty: self.ty,
        }
    }

    pub fn transform(self, transform: Transform) -> Self {
        Self {
            fill: self.fill,
            transform: transform,
            ty: self.ty,
        }
    }

    pub fn translate(mut self, to: Vector2) -> Self {
        self.transform.translate = Some(to);

        Self {
            fill: self.fill,
            transform: self.transform,
            ty: self.ty,
        }
    }

    pub fn rotate(mut self, angle: Scalar) -> Self {
        self.transform.rotate = Some(angle);

        Self {
            fill: self.fill,
            transform: self.transform,
            ty: self.ty,
        }
    }
}

impl Style<Stroke> {
    pub fn stroke(self, stroke: Stroke) -> Self {
        Self {
            fill: self.fill,
            transform: self.transform,
            ty: Some(stroke),
        }
    }
}

impl Style<Font> {
    pub fn font(self, font: Font) -> Self {
        Self {
            fill: self.fill,
            transform: self.transform,
            ty: Some(font),
        }
    }
}

impl<T: WriteAttributes> WriteAttributes for Style<T> {
    fn write(&self, attributes: &mut svg::Attributes) {
        if let Some(fill) = self.fill {
            attributes.insert(keys::FILL.into(), fill.into());
        }
        if let Some(ref ty) = self.ty {
            ty.write(attributes);
        }

        attributes.insert(keys::TRANSFORM.into(), self.transform.into())
    }
}

/*
impl<T> Default for Style<T> {
    fn default() -> Self {
        Self {
            fill: None,
            transform: None,
            ty: None,
        }
    }
}
*/
