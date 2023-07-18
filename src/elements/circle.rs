use crate::anchor::{Anchor, AnchorT};
use crate::style::Stroke;
use crate::svg::{keys, Attributes, Circle as SvgCircle, IntoElem, ToAttributes};
use crate::{into_elem, Scalar, Vector2};

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

into_elem!(Circle, SvgCircle, Stroke);

impl ToAttributes for Circle {
    fn to_attributes(&self, attributes: &mut Attributes) {
        attributes.insert(keys::CX.into(), self.origin[0].into());
        attributes.insert(keys::CY.into(), self.origin[1].into());
        attributes.insert(keys::RADIUS.into(), self.radius.into());
    }
}

impl AnchorT for Circle {
    fn anchor(&self, anchor: Anchor) -> Vector2 {
        // positive X is right (east)
        // positive Y is up (north)
        let (radius, angle) = match anchor {
            Anchor::Origin => return self.origin,
            Anchor::North => return self.origin + Vector2::y() * self.radius,
            Anchor::East => return self.origin + Vector2::x() * self.radius,
            Anchor::South => return self.origin + Vector2::y() * (-self.radius),
            Anchor::West => return self.origin + Vector2::x() * (-self.radius),
            Anchor::NorthEast => (self.radius, 45.0),
            Anchor::SouthEast => (self.radius, -45.0),
            Anchor::SouthWest => (self.radius, -135.0),
            Anchor::NorthWest => (self.radius, 135.0),
            Anchor::Polar { radius, angle } => (radius, angle),
        };

        self.origin + polar_coordinates(radius, angle)
    }
}

fn polar_coordinates(radius: Scalar, angle: Scalar) -> Vector2 {
    let radians = angle * crate::PI / 180.0;
    let (s, c) = radians.sin_cos();
    Vector2::new(radius * c, radius * s)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn to_attributes() {
        let mut attributes = Attributes::new();
        let circle = Circle::new();
        circle.to_attributes(&mut attributes);

        assert_eq!(attributes.get(keys::CX).unwrap().clone().deref(), "0");
        assert_eq!(attributes.get(keys::CY).unwrap().clone().deref(), "0");
        assert_eq!(attributes.get(keys::RADIUS).unwrap().clone().deref(), "0");

        let circle = Circle::new().at(Vector2::new(12.0, -32.0)).radius(10.0);
        circle.to_attributes(&mut attributes);

        assert_eq!(attributes.get(keys::CX).unwrap().clone().deref(), "12");
        assert_eq!(attributes.get(keys::CY).unwrap().clone().deref(), "-32");
        assert_eq!(attributes.get(keys::RADIUS).unwrap().clone().deref(), "10");
    }

    #[test]
    fn anchors() {
        let radius = 10.0;
        let (s, c) = (crate::PI / 4.0).sin_cos(); //45 degrees
        let yr = s * radius;
        let xr = c * radius;

        let circle = Circle::new().radius(radius);
        assert_eq!(circle.origin(), Vector2::zeros());
        assert_eq!(circle.north(), Vector2::new(0.0, radius));
        assert_eq!(circle.northeast(), Vector2::new(xr, yr));
        assert_eq!(circle.east(), Vector2::new(radius, 0.0));
        assert_eq!(circle.southeast(), Vector2::new(xr, -yr));
        assert_eq!(circle.south(), Vector2::new(0.0, -radius));
        assert_eq!(circle.southwest(), Vector2::new(-xr, -yr));
        assert_eq!(circle.west(), Vector2::new(-radius, 0.0));
        assert_eq!(circle.northwest(), Vector2::new(-xr, yr));

        assert_eq!(circle.above(10.0), Vector2::new(0.0, 20.0));
        assert_eq!(circle.below(10.0), Vector2::new(0.0, -20.0));
        assert_eq!(circle.left(10.0), Vector2::new(-20.0, 0.0));
        assert_eq!(circle.right(10.0), Vector2::new(20.0, 0.0));

        assert_eq!(
            circle.above_right(5.0, 2.0),
            circle.northeast() + Vector2::new(5.0, 2.0)
        );
        assert_eq!(
            circle.above_left(2.0, 1.0),
            circle.northwest() + Vector2::new(-2.0, 1.0)
        );
        assert_eq!(
            circle.below_left(1.0, 9.0),
            circle.southwest() + Vector2::new(-1.0, -9.0)
        );
        assert_eq!(
            circle.below_right(-9.0, 4.0),
            circle.southeast() + Vector2::new(-9.0, -4.0)
        );

        let anchor = Anchor::Polar {
            radius: 2.0 * radius,
            angle: 135.0,
        };
        assert!((circle.anchor(anchor) - 2.0 * circle.northwest()).norm() < 1e-6);
    }
}
