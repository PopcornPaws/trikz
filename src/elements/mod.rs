mod circle;
mod document;
mod line;
mod rectangle;

use circle::Circle;
pub use document::Document;
use line::Line;
use rectangle::Rectangle;

use crate::style::Style;
use crate::svgutils::{raw::{self, Node}, ToAttributes};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

type ElemRef = Rc<RefCell<raw::Element>>;

pub struct Element<T> {
    elem: ElemRef,
    ty: PhantomData<T>,
}

impl<T> Clone for Element<T> {
    fn clone(&self) -> Self {
        Self {
            elem: Rc::clone(&self.elem),
            ty: PhantomData,
        }
    }
}

impl<T> Element<T> {
    pub fn new(elem: ElemRef) -> Self {
        Self {
            elem,
            ty: PhantomData,
        }
    }

    pub fn insert<V: Into<raw::Value>>(&self, key: String, value: V) {
        let cloned_ref = Rc::clone(&self.elem);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(key, value.into());
    }

    /// Clones specifically the underlying data behind an Rc and not the Rc itself.
    pub fn like(self, other: Self) -> Self {
        self.elem
            .as_ref()
            .replace(other.elem.as_ref().borrow().clone());
        self
    }

    pub fn insert_multi<I, V>(&self, iter: I)
    where
        V: Into<raw::Value>,
        I: Iterator<Item = (String, V)>,
    {
        let cloned_ref = Rc::clone(&self.elem);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.extend(iter.map(|(k, v)| (k, v.into())));
    }
}

impl<T: ReprT> Element<T> {
    pub fn with_style(self, style: &Style<T::Repr>) -> Self {
        let cloned_ref = Rc::clone(&self.elem);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        style.to_attributes(attributes);
        self
    }
}

pub trait ReprT {
    type Repr: ToAttributes;
}
