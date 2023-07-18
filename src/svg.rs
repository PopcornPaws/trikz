use crate::style::Style;

pub use svglib::node::element::*;
pub use svglib::node::{Attributes, Node, Value};

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
}

// TODO
// pub struct ElemBuilder<T>(T); // where T: Node
//
// impl From<WrapperType> for ElemBuilder {
//  ...
// }
//
// impl<T> ElemBuilder<T> {
//  pub fn style(&mut self, style: &Style) -> &mut Self {
//      ...
//  }
//  pub fn transform(&mut self, transform: ... ) -> &mut Self { // not necessary
//  }
//  pub fn translate(&mut self, translation: Vector2) -> &mut Self {
//  }
//  pub fn rotate(&mut self, angle: Scalar) -> &mut Self {
//  }
//  pub fn finalize(&mut self) -> T
// }

pub trait IntoElem {
    type Output: Sized + Node;
    type StyleType: WriteAttributes;
    fn into_elem(self, style: &Style<Self::StyleType>) -> Self::Output;
}

pub trait WriteAttributes {
    fn write(&self, attributes: &mut Attributes);
}
