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
