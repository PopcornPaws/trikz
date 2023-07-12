use crate::{Scalar, Vector2};

#[derive(Clone)]
pub struct Rectangle {
    pub attributes: Attributes,
    pub origin: Vector2,
    pub style: crate::Style,
}

#[derive(Clone, Copy)]
pub struct Attributes {
    pub width: Scalar,
    pub height: Scalar,
    pub corner_radius: Scalar,
}

impl Rectangle {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes,
            origin: Vector2::zeros(),
        }
    }

    pub fn at(self, origin: Vector2) -> Self {
        Self {
            attributes: self.attributes,
            origin,
        }
    }
}
