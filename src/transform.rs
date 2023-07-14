use crate::style::Style;

pub mod svg {
    pub use svg::node::{Attributes, Value, Node};
    pub use svg::node::element::*;
}

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
}

pub trait Transform {
    type Output: Sized + svg::Node;
    type StyleType: WriteAttribute;
    fn into_svg(self, style: &Style<Self::StyleType>) -> Self::Output;
}

pub trait WriteAttribute {
    fn write(&self, attributes: &mut svg::Attributes);
}
