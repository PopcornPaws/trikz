use super::{Element, ReprT};
use crate::anchor::{anchor_rectangle, Anchor, AnchorT};
use crate::svgutils::keys;
use crate::{Scalar, Vector2};

pub struct Rectangle;

impl ReprT for Rectangle {
    type Repr = crate::style::Stroke;
}

struct Geometry {
    origin: Vector2,
    height: Scalar,
    width: Scalar,
}

impl Element<Rectangle> {
    pub fn at(self, mut origin: Vector2) -> Self {
        let height: Scalar = self.get(keys::HEIGHT);
        let width: Scalar = self.get(keys::WIDTH);
        origin -= Vector2::new(width / 2.0, height / 2.0);
        self.insert_multi([keys::X, keys::Y].into_iter().zip(origin.iter().copied()));
        self
    }

    pub fn width(self, width: Scalar) -> Self {
        let x: Scalar = self.get(keys::X);
        self.insert(keys::X, x - width / 2.0);
        self.insert(keys::WIDTH, width);
        self
    }

    pub fn height(self, height: Scalar) -> Self {
        let y: Scalar = self.get(keys::Y);
        self.insert(keys::Y, y - height / 2.0);
        self.insert(keys::HEIGHT, height);
        self
    }

    pub fn rounded_corners(self, corner_radius: Scalar) -> Self {
        self.insert(keys::CORNER_RADIUS, corner_radius);
        self
    }

    fn geometry(&self) -> Geometry {
        let x: Scalar = self.get(keys::X);
        let y: Scalar = self.get(keys::Y);
        let height = self.get(keys::HEIGHT);
        let width = self.get(keys::WIDTH);
        Geometry {
            origin: Vector2::new(x + width / 2.0, y + height / 2.0),
            height,
            width,
        }
    }
}

impl AnchorT for Element<Rectangle> {
    fn anchor(&self, anchor: Anchor) -> Vector2 {
        let geometry = self.geometry();
        anchor_rectangle(
            anchor,
            geometry.origin,
            geometry.width / 2.0,
            geometry.height / 2.0,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::svgutils::raw;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    #[test]
    fn create_and_modify() {
        let elem = Rc::new(RefCell::new(raw::Rectangle::new().deref().clone()));

        let rect = Element::<Rectangle>::new(Rc::clone(&elem))
            .width(1.5)
            .height(2.0);

        let geometry = rect.geometry();
        assert_eq!(geometry.height, 2.0);
        assert_eq!(geometry.width, 1.5);
        assert_eq!(geometry.origin, Vector2::zeros());

        let other_elem = Rc::new(RefCell::new(raw::Rectangle::new().deref().clone()));
        let other_rect = Element::<Rectangle>::new(Rc::clone(&other_elem))
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
}
