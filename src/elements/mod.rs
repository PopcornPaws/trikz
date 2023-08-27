mod arrow;
mod circle;
mod document;
mod line;
mod marker;
mod path;
mod rectangle;

use circle::Circle;
pub use document::Document;
use line::Line;
use marker::Marker;
pub use path::PathBuilder;
//use path::Path;
use rectangle::Rectangle;

use crate::style::Style;
use crate::svgutils::{
    raw::{self, Node},
    ToAttributes,
};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

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

    /// Clones specifically the underlying data behind an Rc and not the Rc itself.
    pub fn like(self, other: Self) -> Self {
        self.elem.as_ref().replace(other.to_raw());
        self
    }

    pub fn add_child<C: Node>(&self, child: C) {
        let cloned_ref = Rc::clone(&self.elem);
        let mut element = cloned_ref.borrow_mut();
        let children = element.get_children_mut();
        children.push(Box::new(child));
    }

    pub fn insert<K: Into<String>, V: Into<raw::Value>>(&self, key: K, value: V) {
        let cloned_ref = Rc::clone(&self.elem);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.insert(key.into(), value.into());
    }

    pub fn insert_multi<I, K, V>(&self, iter: I)
    where
        K: Into<String>,
        V: Into<raw::Value>,
        I: Iterator<Item = (K, V)>,
    {
        let cloned_ref = Rc::clone(&self.elem);
        let mut element = cloned_ref.borrow_mut();
        let attributes = element.get_attributes_mut();
        attributes.extend(iter.map(|(k, v)| (k.into(), v.into())));
    }

    pub fn to_raw(&self) -> raw::Element {
        self.elem.as_ref().clone().into_inner()
    }

    pub fn into_raw(self) -> raw::Element {
        Rc::into_inner(self.elem).unwrap().into_inner()
    }

    pub fn get<V: FromStr + Default>(&self, key: &str) -> V {
        let elem = self.elem.borrow();
        let attributes = elem.get_attributes();
        attributes
            .get(key)
            .and_then(|x| V::from_str(x.deref()).ok())
            .unwrap_or_default()
    }

    pub fn get_raw(&self, key: &str) -> String {
        let elem = self.elem.borrow();
        let attributes = elem.get_attributes();
        attributes
            .get(key)
            .map(|x| x.deref())
            .unwrap_or_default()
            .into()
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
