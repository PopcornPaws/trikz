use crate::anchor::{Anchor, AnchorT};
use crate::style::Stroke;
use crate::svgutils::{keys, Attributes, IntoElem, Rectangle as SvgRectangle, ToAttributes};
use crate::{into_elem, Scalar, Vector2};

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

into_elem!(Rectangle, SvgRectangle, Stroke);

impl ToAttributes for Rectangle {
    fn to_attributes(&self, attributes: &mut Attributes) {
        attributes.insert(keys::X.into(), self.origin[0].into());
        attributes.insert(keys::Y.into(), self.origin[1].into());
        attributes.insert(keys::WIDTH.into(), self.width.into());
        attributes.insert(keys::HEIGHT.into(), self.height.into());
        attributes.insert(keys::CORNER.into(), self.corner_radius.into());
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
    use std::ops::Deref;

    #[test]
    fn to_attributes() {
        let mut attributes = Attributes::new();

        let rectangle = Rectangle::new()
            .at(Vector2::new(10.0, 20.0))
            .width(10.0)
            .height(100.0)
            .rounded_corners(4.0);
        rectangle.to_attributes(&mut attributes);

        assert_eq!(attributes.get(keys::X).unwrap().clone().deref(), "10");
        assert_eq!(attributes.get(keys::Y).unwrap().clone().deref(), "20");
        assert_eq!(attributes.get(keys::WIDTH).unwrap().clone().deref(), "10");
        assert_eq!(attributes.get(keys::HEIGHT).unwrap().clone().deref(), "100");
        assert_eq!(attributes.get(keys::CORNER).unwrap().clone().deref(), "4");

        let rectangle = Rectangle::new();
        rectangle.to_attributes(&mut attributes);

        assert_eq!(attributes.get(keys::X).unwrap().clone().deref(), "0");
        assert_eq!(attributes.get(keys::Y).unwrap().clone().deref(), "0");
        assert_eq!(attributes.get(keys::WIDTH).unwrap().clone().deref(), "0");
        assert_eq!(attributes.get(keys::HEIGHT).unwrap().clone().deref(), "0");
        assert_eq!(attributes.get(keys::CORNER).unwrap().clone().deref(), "0");
    }

    #[test]
    fn anchors() {
        let rectangle = Rectangle::new().width(8.0).height(6.0);
        assert_eq!(rectangle.origin(), Vector2::zeros());
        assert_eq!(rectangle.north(), Vector2::new(0.0, 3.0));
        assert_eq!(rectangle.northeast(), Vector2::new(4.0, 3.0));
        assert_eq!(rectangle.east(), Vector2::new(4.0, 0.0));
        assert_eq!(rectangle.southeast(), Vector2::new(4.0, -3.0));
        assert_eq!(rectangle.south(), Vector2::new(0.0, -3.0));
        assert_eq!(rectangle.southwest(), Vector2::new(-4.0, -3.0));
        assert_eq!(rectangle.west(), Vector2::new(-4.0, 0.0));
        assert_eq!(rectangle.northwest(), Vector2::new(-4.0, 3.0));

        assert_eq!(rectangle.above(10.0), Vector2::new(0.0, 13.0));
        assert_eq!(rectangle.below(10.0), Vector2::new(0.0, -13.0));
        assert_eq!(rectangle.left(10.0), Vector2::new(-14.0, 0.0));
        assert_eq!(rectangle.right(10.0), Vector2::new(14.0, 0.0));

        assert_eq!(rectangle.above_right(5.0, 5.0), Vector2::new(9.0, 8.0));
        assert_eq!(rectangle.above_left(5.0, 5.0), Vector2::new(-9.0, 8.0));
        assert_eq!(rectangle.below_left(5.0, 5.0), Vector2::new(-9.0, -8.0));
        assert_eq!(rectangle.below_right(5.0, 5.0), Vector2::new(9.0, -8.0));

        // sin(angle) = 3.0 / 5.0;
        // we need it in degrees
        // we should basically get southeast with these polar coordinates
        let angle = -(3.0 / 5.0 as Scalar).asin() * 180.0 / crate::PI;
        let anchor = Anchor::Polar { radius: 5.0, angle };
        assert!((rectangle.anchor(anchor) - rectangle.southeast()).norm() < 1e-6);
    }
}
