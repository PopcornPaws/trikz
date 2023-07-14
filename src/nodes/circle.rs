use crate::style::{Stroke, Style};
use crate::transform::{keys, svg, Transform, WriteAttribute};
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

impl Transform for Circle {
    type Output = svg::Circle;
    type StyleType = Stroke;
    fn into_svg(self, style: &Style<Self::StyleType>) -> Self::Output {
        let mut output = svg::Circle::new()
            .set(keys::CX, self.origin[0])
            .set(keys::CY, self.origin[1])
            .set(keys::RADIUS, self.radius);

        style.write(output.get_attributes_mut());

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::style::Color;
    use std::ops::Deref;

    #[test]
    fn into_svg() {
        let circle = Circle::new();
        let style = Style::default();
        let svg = circle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(keys::CX).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(keys::CY).unwrap().clone().deref(), "0");
        assert_eq!(
            svg_attributes.get(keys::RADIUS).unwrap().clone().deref(),
            "0"
        );
        assert!(svg_attributes.get(keys::FILL).is_none());
        assert!(svg_attributes.get(keys::STROKE).is_none());

        let circle = Circle::new().at(Vector2::new(12.0, -32.0)).radius(10.0);
        let style = Style::new().fill(Color::White);
        let svg = circle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(keys::CX).unwrap().clone().deref(), "12");
        assert_eq!(svg_attributes.get(keys::CY).unwrap().clone().deref(), "-32");
        assert_eq!(
            svg_attributes.get(keys::RADIUS).unwrap().clone().deref(),
            "10"
        );
        assert_eq!(svg_attributes.get(keys::FILL).unwrap().clone().deref(), "white");
        assert!(svg_attributes.get(keys::STROKE).is_none());
    }
}
