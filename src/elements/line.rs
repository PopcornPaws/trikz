use super::{Element, ReprT};
use crate::svgutils::keys;
use crate::Vector2;

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
}
