use crate::into_svg::{node, IntoSvg, CX, CY, RADIUS, STYLE};
use crate::style::Style;
use crate::{Scalar, Vector2};

#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
    pub origin: Vector2,
    pub radius: Scalar,
}

impl Circle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(self, origin: Vector2) -> Self {
        Self {
            origin,
            radius: self.radius,
        }
    }

    pub fn radius(self, radius: Scalar) -> Self {
        Self {
            origin: self.origin,
            radius,
        }
    }
}

impl IntoSvg for Circle {
    type Output = node::Circle;
    fn into_svg(self, style: &Style) -> Self::Output {
        node::Circle::new()
            .set(CX, self.origin[0])
            .set(CY, self.origin[1])
            .set(RADIUS, self.radius)
            .set(STYLE, style.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn into_svg() {
        let circle = Circle::new();
        let style = Style::default();
        let svg = circle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(CX).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(CY).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(RADIUS).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(STYLE).unwrap().clone().deref(), style.to_string());

        let circle = Circle::new().at(Vector2::new(12.0, -32.0)).radius(10.0);
        let svg = circle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(CX).unwrap().clone().deref(), "12");
        assert_eq!(svg_attributes.get(CY).unwrap().clone().deref(), "-32");
        assert_eq!(svg_attributes.get(RADIUS).unwrap().clone().deref(), "10");
        assert_eq!(svg_attributes.get(STYLE).unwrap().clone().deref(), style.to_string());
    }
}
