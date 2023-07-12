#[derive(Clone)]
pub struct Circle {
    pub attributes: Attributes,
    pub origin: Vector2,
    pub style: crate::Style,
}

#[derive(Clone, Copy)]
pub struct Attributes {
    radius: Scalar,
}

impl Rectangle {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes,
            origin: Vector2::zeros(),
        }
    }
}
