mod circle;
mod rectangle;

use circle::Circle;
use rectangle::Rectangle;

use crate::svgutils::raw::{self, Node};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct Document {
    elements: Vec<Rc<RefCell<raw::Element>>>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn circle(&mut self) -> Circle {
        // cloning is cheap because the rectangle is empty
        // but we need it as an Element, not as a rectangle type
        let elem = raw::Circle::new().deref().clone();
        self.elements.push(Rc::new(RefCell::new(elem)));
        let index = self.elements.len() - 1;
        Circle::new(Rc::clone(&self.elements[index]))
    }

    pub fn rectangle(&mut self) -> Rectangle {
        // cloning is cheap because the rectangle is empty
        // but we need it as an Element, not as a rectangle type
        let elem = raw::Rectangle::new().deref().clone();
        self.elements.push(Rc::new(RefCell::new(elem)));
        let index = self.elements.len() - 1;
        Rectangle::new(Rc::clone(&self.elements[index]))
    }

    pub fn finalize(self) -> raw::Document {
        let mut document = raw::Document::new();
        self.elements.into_iter().for_each(|elem| {
            document.append(Rc::into_inner(elem).unwrap().into_inner());
        });
        document
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

// TODO
//macro_rules! elements {
// implement circle, rectangle, etc...
//}
