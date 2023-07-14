use crate::{Scalar, Vector2};
use crate::style::Style;

#[derive(Clone)]
pub struct Rectangle {
    pub attributes: Attributes,
    pub origin: Vector2,
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

#[derive(Clone, Copy, Debug)]
pub struct Attributes {
    pub width: Scalar,
    pub height: Scalar,
    pub corner_radius: Scalar,
}

impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            width,
            height: self.height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn height(self, height: Scalar) -> Self {
        Self {
            width: self.width,
            height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn radius(self, corner_radius: Scalar) -> Self {
        Self {
            width: self.width,
            height: self.height,
            corner_radius,
        }
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 5.0,
            corner_radius: 0.0,
        }
    }
}


// TODO
// - impl AnchorT
