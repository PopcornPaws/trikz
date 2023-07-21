use crate::anchor::{Anchor, AnchorT};
use crate::style::{Stroke, Style};
use crate::svgutils::{keys, raw, ToAttributes};
use crate::{Scalar, Vector2};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

pub struct Rectangle(Rc<RefCell<raw::Element>>);

struct Geometry {
    origin: Vector2,
    height: Scalar,
    width: Scalar,
}

impl Rectangle {
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
        attributes.insert(keys::X.into(), origin[0].into());
        attributes.insert(keys::Y.into(), origin[1].into());
        self
    }

    pub fn width(self, width: Scalar) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::WIDTH.into(), width.into());
        self
    }

    pub fn height(self, height: Scalar) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::HEIGHT.into(), height.into());
        self
    }

    pub fn rounded_corners(self, corner_radius: Scalar) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::CORNER_RADIUS.into(), corner_radius.into());
        self
    }

    pub fn with_style(&self, style: &Style<Stroke>) {
        let mut element = self.0.borrow_mut();
        let attributes = element.get_attributes_mut();
        style.to_attributes(attributes);
    }

    fn geometry(&self) -> Geometry {
        let element = self.0.borrow();
        let attributes = element.get_attributes();

        let x = attributes
            .get(keys::X)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let y = attributes
            .get(keys::Y)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let height = attributes
            .get(keys::HEIGHT)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let width = attributes
            .get(keys::WIDTH)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        Geometry {
            origin: Vector2::new(x, y),
            height,
            width,
        }
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl AnchorT for Rectangle {
    fn anchor(&self, anchor: Anchor) -> Vector2 {
        // positive X is right (east)
        // positive Y is up (north)
        let geometry = self.geometry();
        match anchor {
            Anchor::Origin => geometry.origin,
            Anchor::North => geometry.origin + Vector2::new(0.0, geometry.height / 2.0),
            Anchor::NorthEast => {
                geometry.origin + Vector2::new(geometry.width / 2.0, geometry.height / 2.0)
            }
            Anchor::East => geometry.origin + Vector2::new(geometry.width / 2.0, 0.0),
            Anchor::SouthEast => {
                geometry.origin + Vector2::new(geometry.width / 2.0, -geometry.height / 2.0)
            }
            Anchor::South => geometry.origin + Vector2::new(0.0, -geometry.height / 2.0),
            Anchor::SouthWest => {
                geometry.origin + Vector2::new(-geometry.width / 2.0, -geometry.height / 2.0)
            }
            Anchor::West => geometry.origin + Vector2::new(-geometry.width / 2.0, 0.0),
            Anchor::NorthWest => {
                geometry.origin + Vector2::new(-geometry.width / 2.0, geometry.height / 2.0)
            }
            Anchor::Polar { radius, angle } => {
                let radians = angle * crate::PI / 180.0;
                let (s, c) = radians.sin_cos();
                geometry.origin + Vector2::new(radius * c, radius * s)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_modify() {
        let elem = Rc::new(RefCell::new(raw::Rectangle::new().deref().clone()));

        let rect = Rectangle::new(Rc::clone(&elem)).width(1.5).height(2.0);

        let geometry = rect.geometry();
        assert_eq!(geometry.height, 2.0);
        assert_eq!(geometry.width, 1.5);
        assert_eq!(geometry.origin, Vector2::zeros());

        let other_elem = Rc::new(RefCell::new(raw::Rectangle::new().deref().clone()));
        let other_rect = Rectangle::new(Rc::clone(&other_elem))
            .like(rect.clone())
            .at(Vector2::new(10.0, 20.0))
            .rounded_corners(0.5);

        // check the original hasn't changed
        let geometry = rect.geometry();
        assert_eq!(geometry.height, 2.0);
        assert_eq!(geometry.width, 1.5);
        assert_eq!(geometry.origin, Vector2::zeros());

        // check the other one
        let geometry = other_rect.geometry();
        assert_eq!(geometry.height, 2.0);
        assert_eq!(geometry.width, 1.5);
        assert_eq!(geometry.origin, Vector2::new(10.0, 20.0));

        // check rounded corners
        assert_eq!(
            other_elem
                .borrow()
                .get_attributes()
                .get(keys::CORNER_RADIUS)
                .unwrap()
                .deref(),
            "0.5"
        );
    }

    #[test]
    fn anchors() {
        let elem = raw::Rectangle::new().deref().clone();
        let rectangle = Rectangle::new(Rc::new(RefCell::new(elem)))
            .width(8.0)
            .height(6.0);
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
