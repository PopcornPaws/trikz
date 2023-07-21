use crate::style::Style;

pub use svg::node::element::*;
pub use svg::node::{Attributes, Node, Value};

use std::marker::PhantomData;
use std::ops::DerefMut;

pub mod keys {
    pub const X: &str = "x";
    pub const Y: &str = "y";

    // rectangle attributes
    pub const WIDTH: &str = "width";
    pub const HEIGHT: &str = "height";
    pub const CORNER: &str = "rx";

    // circle attributes
    pub const CX: &str = "cx";
    pub const CY: &str = "cy";
    pub const RADIUS: &str = "r";

    // style
    pub const FILL: &str = "fill";
    pub const STROKE: &str = "stroke";
    pub const STROKE_OPACITY: &str = "stroke-opacity";
    pub const STROKE_WIDTH: &str = "stroke-width";
    pub const STROKE_STYLE: &str = "stroke-dasharray";

    // font
    pub const FONT_SIZE: &str = "font-size";

    // transform
    pub const TRANSFORM: &str = "transform";

    // path
    pub const PATH: &str = "d";

    // line
    pub const X1: &str = "x1";
    pub const X2: &str = "x2";
    pub const Y1: &str = "y1";
    pub const Y2: &str = "y2";

    // marker
    pub const MARKERS: &[&str] = &["marker-start", "marker-mid", "marker-end"];
    pub const MARKER_ID: &str = "id";
    pub const MARKER_HEIGHT: &str = "markerHeight";
    pub const MARKER_WIDTH: &str = "markerWidth";
    pub const MARKER_ORIENT: &str = "orient";
}

pub struct ElemBuilder<E, R> {
    elem: E,
    repr: PhantomData<R>,
}

impl<E, R, T> From<T> for ElemBuilder<E, R>
where
    T: IntoElem<Elem = E, Repr = R>,
{
    fn from(wrapper: T) -> Self {
        Self {
            elem: wrapper.into_elem(),
            repr: PhantomData,
        }
    }
}

// TODO should transformation (translation + rotation) methods be included in this impl, or on each
// element? see src/style/transform.rs it should probably be a trait
impl<E, R> ElemBuilder<E, R>
where
    E: DerefMut<Target = Element>,
    R: ToAttributes,
{
    pub fn with_style(mut self, style: &Style<R>) -> Self {
        style.to_attributes(self.elem.deref_mut().get_attributes_mut());
        self
    }

    pub fn finalize(self) -> E {
        self.elem
    }
}

pub trait IntoElem: ToAttributes {
    type Elem: DerefMut<Target = Element>;
    type Repr: ToAttributes;

    fn into_elem(self) -> Self::Elem;
}

pub trait ToAttributes {
    fn to_attributes(&self, attributes: &mut Attributes);
}

impl ToAttributes for () {
    fn to_attributes(&self, _attributes: &mut Attributes) {}
}
