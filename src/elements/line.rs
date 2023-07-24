use super::{Element, ReprT};
use crate::anchor::{Anchor, AnchorT};
use crate::svgutils::keys;
use crate::{Scalar, Vector2};
use std::ops::Deref;
use std::str::FromStr;

pub struct Line;

impl ReprT for Line {
    type Repr = crate::style::Stroke;
}

impl Element<Line> {
    pub fn start(self, start: Vector2) -> Self {
        self.insert_multi(
            [keys::X1.into(), keys::Y1.into()]
                .into_iter()
                .zip(start.iter().copied()),
        );
        self
    }

    pub fn end(self, end: Vector2) -> Self {
        self.insert_multi(
            [keys::X2.into(), keys::Y2.into()]
                .into_iter()
                .zip(end.iter().copied()),
        );
        self
    }

    pub fn marker(self, id: String) -> Self {
        todo!("{id}")
    }

    fn geometry(&self) -> Geometry {
        let element = self.elem.borrow();
        let attributes = element.get_attributes();

        let x1 = attributes
            .get(keys::X1)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let y1 = attributes
            .get(keys::Y1)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let x2 = attributes
            .get(keys::X2)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let y2 = attributes
            .get(keys::Y2)
            .and_then(|x| Scalar::from_str(x.deref()).ok())
            .unwrap_or_default();

        let start = Vector2::new(x1, y1);
        let end = Vector2::new(x2, y2);

        Geometry { start, end }
    }
}

struct Geometry {
    start: Vector2,
    end: Vector2,
}

impl AnchorT for Element<Line> {
    fn anchor(&self, anchor: Anchor) -> Self {
        let geometry = self.geometry();
        let origin = (geometry.start + geometry.end) / 2.0;
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
        let elem = Rc::new(RefCell::new(raw::Line::new().deref().clone()));
        let _ = Element::<Line>::new(Rc::clone(&elem))
            .start(Vector2::new(-2.0, 3.0))
            .end(Vector2::new(5.0, 6.5));

        assert_eq!(
            elem.borrow()
                .get_attributes()
                .get(keys::X1)
                .unwrap()
                .deref(),
            "-2"
        );
        assert_eq!(
            elem.borrow()
                .get_attributes()
                .get(keys::Y1)
                .unwrap()
                .deref(),
            "3"
        );
        assert_eq!(
            elem.borrow()
                .get_attributes()
                .get(keys::X2)
                .unwrap()
                .deref(),
            "5"
        );
        assert_eq!(
            elem.borrow()
                .get_attributes()
                .get(keys::Y2)
                .unwrap()
                .deref(),
            "6.5"
        );
    }

    #[test]
    fn geometry() {
        let orig = Rc::new(RefCell::new(raw::Line::new().deref().clone()));
        let elem = Element::<Line>::new(Rc::clone(&orig));

        let geometry = elem.geometry();
        assert_eq!(geometry.start, Vector2::zeros());
        assert_eq!(geometry.end, Vector2::zeros());

        elem.insert_multi(
            [keys::X1.into(), keys::Y1.into(), keys::X2.into(), keys::Y2.into()]
                .into_iter()
                .zip([0.0, 1.0, 10.0, 2.0]),
        );

        let geometry = elem.geometry();
        assert!((geometry.start - Vector2::new(0.0, 1.0)).norm() < 1e-6);
        assert!((geometry.end - Vector2::new(10.0, 2.0)).norm() < 1e-6);

        elem.insert_multi(
            [keys::X1.into(), keys::Y1.into(), keys::X2.into(), keys::Y2.into()]
                .into_iter()
                .zip([10.0, -1.0, -5.0, 2.0]),
        );

        let geometry = elem.geometry();
        assert!((geometry.start - Vector2::new(10.0, -1.0)).norm() < 1e-6);
        assert!((geometry.end - Vector2::new(-5.0, 2.0)).norm() < 1e-6);
    }
}
