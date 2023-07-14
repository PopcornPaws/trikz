use crate::anchor::{AnchorT, Anchor};
use crate::style::{Stroke, Style};
use crate::transform::{keys, svg, Transform, WriteAttribute};
use crate::{Scalar, Vector2};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
    pub origin: Vector2,
    pub width: Scalar,
    pub height: Scalar,
    pub corner_radius: Scalar,
}

impl Rectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(self, origin: Vector2) -> Self {
        Self {
            origin,
            width: self.width,
            height: self.height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            origin: self.origin,
            width,
            height: self.height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn height(self, height: Scalar) -> Self {
        Self {
            origin: self.origin,
            width: self.width,
            height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn rounded_corners(self, corner_radius: Scalar) -> Self {
        Self {
            origin: self.origin,
            width: self.width,
            height: self.height,
            corner_radius,
        }
    }
}

impl Transform for Rectangle {
    type Output = svg::Rectangle;
    type StyleType = Stroke;
    fn into_svg(self, style: &Style<Self::StyleType>) -> Self::Output {
        let mut output = svg::Rectangle::new()
            .set(keys::X, self.origin[0])
            .set(keys::Y, self.origin[1])
            .set(keys::WIDTH, self.width)
            .set(keys::HEIGHT, self.height)
            .set(keys::CORNER, self.corner_radius);

        style.write(output.get_attributes_mut());

        output
    }
}

impl AnchorT for Rectangle {
    fn anchor(&self, anchor: Anchor) -> Vector2 {
        // positive X is right (east)
        // positive Y is up (north)
        match anchor {
            Anchor::Origin => self.origin,
            Anchor::North => self.origin + Vector2::new(0.0, self.height / 2.0),
            Anchor::NorthEast => self.origin + Vector2::new(self.width / 2.0, self.height / 2.0),
            Anchor::East => self.origin + Vector2::new(self.width / 2.0, 0.0),
            Anchor::SouthEast => self.origin + Vector2::new(self.width / 2.0, -self.height / 2.0),
            Anchor::South => self.origin + Vector2::new(0.0, -self.height / 2.0),
            Anchor::SouthWest => self.origin + Vector2::new(-self.width / 2.0, -self.height / 2.0),
            Anchor::West => self.origin + Vector2::new(-self.width / 2.0, 0.0),
            Anchor::NorthWest => self.origin + Vector2::new(-self.width / 2.0, self.height / 2.0),
            Anchor::Polar { radius, angle } => {
                let radians = angle * crate::PI / 180.0;
                let (s, c) = radians.sin_cos();
                self.origin + Vector2::new(radius * c, radius * s)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::style::{Color, Stroke};

    use std::ops::Deref;

    #[test]
    fn into_svg() {
        let rectangle = Rectangle::new()
            .at(Vector2::new(10.0, 20.0))
            .width(10.0)
            .height(100.0)
            .rounded_corners(4.0);

        let style = Style::default();

        let svg = rectangle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(keys::X).unwrap().clone().deref(), "10");
        assert_eq!(svg_attributes.get(keys::Y).unwrap().clone().deref(), "20");
        assert_eq!(
            svg_attributes.get(keys::WIDTH).unwrap().clone().deref(),
            "10"
        );
        assert_eq!(
            svg_attributes.get(keys::HEIGHT).unwrap().clone().deref(),
            "100"
        );
        assert_eq!(
            svg_attributes.get(keys::CORNER).unwrap().clone().deref(),
            "4"
        );
        assert!(svg_attributes.get(keys::FILL).is_none());
        assert!(svg_attributes.get(keys::STROKE).is_none());

        let rectangle = Rectangle::new();

        let stroke = Stroke::new().dashed().color(Color::Magenta);
        let style = Style::new().fill(Color::Green).stroke(stroke);

        let svg = rectangle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(keys::X).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(keys::Y).unwrap().clone().deref(), "0");
        assert_eq!(
            svg_attributes.get(keys::WIDTH).unwrap().clone().deref(),
            "0"
        );
        assert_eq!(
            svg_attributes.get(keys::HEIGHT).unwrap().clone().deref(),
            "0"
        );
        assert_eq!(
            svg_attributes.get(keys::CORNER).unwrap().clone().deref(),
            "0"
        );
        assert_eq!(
            svg_attributes.get(keys::FILL).unwrap().clone().deref(),
            "green"
        );
        assert_eq!(
            svg_attributes.get(keys::STROKE).unwrap().clone().deref(),
            "magenta"
        );
    }
}
