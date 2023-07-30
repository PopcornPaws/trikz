use super::{Element, ReprT};
use crate::anchor::{anchor_circle, Anchor, AnchorT};
use crate::svgutils::keys;
use crate::{Scalar, Vector2};

pub struct Circle;

impl ReprT for Circle {
    type Repr = crate::style::Stroke;
}

struct Geometry {
    origin: Vector2,
    radius: Scalar,
}

impl Element<Circle> {
    pub fn at(self, origin: Vector2) -> Self {
        self.insert_multi([keys::CX, keys::CY].into_iter().zip(origin.iter().copied()));
        self
    }

    pub fn radius(self, radius: Scalar) -> Self {
        self.insert(keys::RADIUS, radius);
        self
    }

    fn geometry(&self) -> Geometry {
        let x = self.get(keys::CX);
        let y = self.get(keys::CY);
        let radius = self.get(keys::RADIUS);

        Geometry {
            origin: Vector2::new(x, y),
            radius,
        }
    }
}

impl AnchorT for Element<Circle> {
    fn anchor(&self, anchor: Anchor) -> Vector2 {
        let geometry = self.geometry();
        anchor_circle(anchor, geometry.origin, geometry.radius)
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
        let elem = Rc::new(RefCell::new(raw::Circle::new().deref().clone()));
        let circ = Element::<Circle>::new(Rc::clone(&elem)).radius(5.0);

        let geometry = circ.geometry();
        assert_eq!(geometry.origin, Vector2::zeros());
        assert_eq!(geometry.radius, 5.0);

        let other_elem = Rc::new(RefCell::new(raw::Circle::new().deref().clone()));
        let other_circ = Element::<Circle>::new(Rc::clone(&other_elem))
            .like(circ)
            .at(Vector2::new(12.0, -32.5));

        let geometry = other_circ.geometry();
        assert_eq!(geometry.origin, Vector2::new(12.0, -32.5));
        assert_eq!(geometry.radius, 5.0);
    }
}
