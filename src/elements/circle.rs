use super::{Element, ReprT};
use crate::anchor::{Anchor, AnchorT, anchor_circle};
use crate::svgutils::keys;
use crate::{Scalar, Vector2};
use std::ops::Deref;
use std::str::FromStr;

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
        self.insert_multi(
            [keys::CX.into(), keys::CY.into()]
                .into_iter()
                .zip(origin.iter().copied()),
        );
        self
    }

    pub fn radius(self, radius: Scalar) -> Self {
        self.insert(keys::RADIUS.into(), radius);
        self
    }

    fn geometry(&self) -> Geometry {
        let element = self.elem.borrow();
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
    use crate::assert_relative_eq;
    use std::cell::RefCell;
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
