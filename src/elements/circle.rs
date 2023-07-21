use crate::anchor::{Anchor, AnchorT};
use crate::style::{Stroke, Style};
use crate::svgutils::{keys, raw, ToAttributes};
use crate::{Scalar, Vector2};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

pub struct Circle(Rc<RefCell<raw::Element>>);

struct Geometry {
    origin: Vector2,
    radius: Scalar,
}

impl Circle {
    pub fn new(inner: Rc<RefCell<raw::Element>>) -> Self {
        Self(inner)
    }

    /// Clones specifically the underlying data behind an Rc and not the Rc itself.
    pub fn like(self, other: Self) -> Self {
        self.0.as_ref().replace(other.0.as_ref().borrow().clone());
        self
    }

    pub fn at(self, origin: Vector2) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::CX.into(), origin[0].into());
        attributes.insert(keys::CY.into(), origin[1].into());
        self
    }

    pub fn radius(self, radius: Scalar) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::RADIUS.into(), radius.into());
        self
    }

    fn geometry(&self) -> Geometry {
        let element = self.0.borrow();
        let attributes = element.get_attributes();

        let x = attributes
            .get(keys::CX)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let y = attributes
            .get(keys::CY)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let radius = attributes
            .get(keys::RADIUS)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        Geometry {
            origin: Vector2::new(x, y),
            radius,
        }
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl AnchorT for Circle {
    fn anchor(&self, anchor: Anchor) -> Vector2 {
        // positive X is right (east)
        // positive Y is up (north)
        let geometry = self.geometry();
        let (radius, angle) = match anchor {
            Anchor::Origin => return geometry.origin,
            Anchor::North => return geometry.origin + Vector2::y() * geometry.radius,
            Anchor::East => return geometry.origin + Vector2::x() * geometry.radius,
            Anchor::South => return geometry.origin + Vector2::y() * (-geometry.radius),
            Anchor::West => return geometry.origin + Vector2::x() * (-geometry.radius),
            Anchor::NorthEast => (geometry.radius, 45.0),
            Anchor::SouthEast => (geometry.radius, -45.0),
            Anchor::SouthWest => (geometry.radius, -135.0),
            Anchor::NorthWest => (geometry.radius, 135.0),
            Anchor::Polar { radius, angle } => (radius, angle),
        };

        geometry.origin + polar_coordinates(radius, angle)
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

    #[test]
    fn create_and_modify() {
        let elem = Rc::new(RefCell::new(raw::Circle::new().deref().clone()));
        let circ = Circle::new(Rc::clone(&elem)).radius(5.0);

        let geometry = circ.geometry();
        assert_eq!(geometry.origin, Vector2::zeros());
        assert_eq!(geometry.radius, 5.0);

        let other_elem = Rc::new(RefCell::new(raw::Circle::new().deref().clone()));
        let other_circ = Circle::new(Rc::clone(&other_elem))
            .like(circ.clone())
            .at(Vector2::new(12.0, -32.5));

        let geometry = other_circ.geometry();
        assert_eq!(geometry.origin, Vector2::new(12.0, -32.5));
        assert_eq!(geometry.radius, 5.0);
    }

    #[test]
    fn anchors() {
        let radius = 10.0;
        let (s, c) = (crate::PI / 4.0).sin_cos(); //45 degrees
        let yr = s * radius;
        let xr = c * radius;

        let elem = Rc::new(RefCell::new(raw::Circle::new().deref().clone()));

        let circle = Circle::new(Rc::clone(&elem)).radius(radius);
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
