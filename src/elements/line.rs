use crate::style::{Stroke, Style};
use crate::svgutils::{keys, raw, ToAttributes};
use crate::Vector2;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Line(Rc<RefCell<raw::Element>>);

impl Line {
    pub fn new(inner: Rc<RefCell<raw::Element>>) -> Self {
        Self(inner)
    }

    pub fn start(self, start: Vector2) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::X1.into(), start[0].into());
        attributes.insert(keys::Y1.into(), start[1].into());
        self
    }

    pub fn end(self, end: Vector2) -> Self {
        let cloned_ref = Rc::clone(&self.0);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(keys::X2.into(), end[0].into());
        attributes.insert(keys::Y2.into(), end[1].into());
        self
    }

    pub fn with_style(&self, style: &Style<Stroke>) {
        let mut element = self.0.borrow_mut();
        let attributes = element.get_attributes_mut();
        style.to_attributes(attributes);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn create_and_modify() {
        let elem = Rc::new(RefCell::new(raw::Line::new().deref().clone()));
        let _ = Line::new(Rc::clone(&elem))
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
